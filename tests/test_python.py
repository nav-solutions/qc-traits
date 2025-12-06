from qc_traits import Epoch, TimeScale, Duration, Polynomial, TimeCorrection, TimeCorrectionsDB

def test_py():
    refepoch = Epoch("2020-01-01T00:00:00 UTC")
    (lhs, rhs) = (TimeScale.UTC, TimeScale.GPST)
    polynomial = Polynomial.from_constant_offset(Duration.from_nanoseconds(10))
    tc = TimeCorrection(lhs, rhs, refepoch, polynomial)
