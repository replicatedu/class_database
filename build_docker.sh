~/.cargo/bin/cargo build
mkdir -p docker_context
cp deploy_key docker_context/
cp Dockerfile docker_context/
cp target/debug/class_database docker_context/
docker build -t db_test_client ./docker_context/.