import subprocess
import sys

output = subprocess.check_output("(cd tests && cargo test)", shell=True)

if b"2 tests" in output:
    pass
else:
    sys.exit("tests did not run correctly")
