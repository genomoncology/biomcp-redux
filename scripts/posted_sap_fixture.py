"""Regenerate the synthetic posted-plan PDF used by the trial document replays."""

from pathlib import Path

from label_history_fixtures import LINES_PER_PAGE, pdf


def main():
    pages = [[f"Synthetic statistical analysis plan, page {n}"] for n in range(1, 25)]
    pages[0][0] = "Statistical Analysis Plan v2.0"
    for page in pages[:2]:
        page.extend(
            f"Table {row:02}: Summarize the fictional study groups and report counts by visit."
            for row in range(1, LINES_PER_PAGE)
        )
    pages[16].extend([
        "Report the Wald chi-square p-value with the model results.",
        "List the estimates and confidence intervals by group.",
    ])
    lines = [
        line
        for page in pages
        for line in page + [""] * (LINES_PER_PAGE - len(page))
    ]
    target = Path(__file__).resolve().parents[1] / "tests/replies/trial/posted-sap/ctgov/synthetic-sap-0.bin"
    target.write_bytes(pdf("\n".join(lines)))


if __name__ == "__main__":
    main()
