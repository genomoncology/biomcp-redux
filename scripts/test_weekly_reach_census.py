"""The weekly job fails with the source and class when measured reach drops."""
from contextlib import redirect_stdout
from io import StringIO
import unittest
from unittest.mock import patch

from reach_census_common import Measurement
from weekly_reach_census import run


class WeeklyReachCensusTest(unittest.TestCase):
    def test_same_floor_passes_then_lower_reach_names_the_class(self):
        floors = {"example": {"rare records": [3, 4]}}
        measured = Measurement(total=40, reached=3, sampled=4)

        def provider(_):
            yield "rare records", measured

        output = StringIO()
        with patch.dict("weekly_reach_census.SOURCES", example=provider), redirect_stdout(output):
            self.assertEqual(run(None, ["example"], floors), 0)
            measured = Measurement(total=40, reached=2, sampled=4)
            self.assertEqual(run(None, ["example"], floors), 1)

        self.assertIn("FAIL example/rare records: reached 2/4, below floor 3/4",
                      output.getvalue())
