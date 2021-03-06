
set -x
set -e

SCRIPT_DIR=$(realpath $(dirname $0))
PROJECT_ROOT=$(dirname $(dirname $SCRIPT_DIR))
TESTS_DIR=$PROJECT_ROOT/polyjuice-tests
DEPS_DIR=$PROJECT_ROOT/integration-test
GODWOKEN_DIR=$DEPS_DIR/godwoken

mkdir -p $DEPS_DIR
if [ -d "$GODWOKEN_DIR" ]
then
    echo "godwoken project already exists"
else
    git clone https://github.com/nervosnetwork/godwoken.git $GODWOKEN_DIR
fi
cd $GODWOKEN_DIR
git checkout 72c2e901078ad0d2b3f7270722e46e5b1cdab67a
git submodule update --init --recursive
cd godwoken-scripts/c
make all-via-docker

cd $PROJECT_ROOT
git submodule update --init --recursive
make all-via-docker

cd $TESTS_DIR
export RUST_BACKTRACE=full
cargo test -- --nocapture
