"""Classify capture replies once for source stopping and comparison eligibility."""
from dataclasses import dataclass
from enum import Enum
import json
from urllib.parse import urlsplit


class Kind(str, Enum):
    SUCCESS = 'success'
    EMPTY = 'empty'
    PROVIDER_FAILURE = 'provider_failure'
    TRANSPORT_FAILURE = 'transport_failure'
    TRUNCATED = 'truncated'
    MALFORMED = 'malformed'
    BUDGET_REFUSAL = 'budget_refusal'
    IDENTITY_REFUSAL = 'identity_refusal'
    LOCAL_REFUSAL = 'local_refusal'


@dataclass(frozen=True)
class Outcome:
    kind: Kind

    @property
    def comparison_eligible(self):
        return self.kind in (Kind.SUCCESS, Kind.EMPTY)

    @property
    def stop_source(self):
        return not self.comparison_eligible


def classify(source, url, status, body, *, truncated=False,
             transport_failure=False, refusal=None):
    if refusal:
        kind = {'budget_exhausted': Kind.BUDGET_REFUSAL,
                'undeclared_identity': Kind.IDENTITY_REFUSAL}.get(refusal, Kind.LOCAL_REFUSAL)
        return Outcome(kind)
    if transport_failure:
        return Outcome(Kind.TRANSPORT_FAILURE)
    if truncated:
        return Outcome(Kind.TRUNCATED)
    try:
        value = json.loads(body)
    except (ValueError, UnicodeError):
        return Outcome(Kind.MALFORMED)
    endpoint = urlsplit(url)
    error = value.get('error') if isinstance(value, dict) else None
    graphql = (source, endpoint.scheme, endpoint.netloc, endpoint.path) in (
        ('civic', 'https', 'civicdb.org', '/api/graphql'),
        ('opentargets', 'https', 'api.platform.opentargets.org', '/api/v4/graphql'),
    )
    if graphql and isinstance(value, dict) and value.get('errors'):
        return Outcome(Kind.PROVIDER_FAILURE)
    if (source == 'openfda' and endpoint.scheme == 'https'
            and endpoint.netloc == 'api.fda.gov'
            and endpoint.path in ('/drug/label.json', '/drug/event.json')
            and status == 404 and isinstance(error, dict)
            and error.get('code') == 'NOT_FOUND'
            and error.get('message') in ('No matches found!', 'No matches found')):
        return Outcome(Kind.EMPTY)
    if not 200 <= status < 300 or error is not None:
        return Outcome(Kind.PROVIDER_FAILURE)
    return Outcome(Kind.SUCCESS)


def comparison_eligible(exit_code, rows):
    return exit_code == 0 and all(
        Outcome(Kind(row['outcome'])).comparison_eligible for row in rows)
