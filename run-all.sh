#!/bin/bash
echo -e " ::::::::::::: Launching the virtual environment... :::::::::::::\n"

# create the virtual environment if it doesn't exist
if [ ! -d ".venv" ]; then
    python3 -m venv .venv
fi
# activate the virtual environment
source .venv/bin/activate
# install the dependencies
echo -e "\n<==========Installing the dependencies...===============>\n"
# Check and install dependencies only if needed
if ! python3 -c "import opencv-python" &>/dev/null; then
    echo -e "\n<==========Installing opencv-python===============>\n"
    pip install opencv-python
else
    echo "opencv-python is already installed in the virtual environment"
fi

if ! python3 -c "import tensorflow" &>/dev/null; then
    echo -e "\n<==========Installing tensorflow :::::::::::::\n"
    python3 -m pip install tensorflow
else
    echo "tensorflow is already installed in the virtual environment"
fi

# if everything is good, run the following commands
if [[ $? -eq 0 ]]; then
    echo -e "\n ::::::::::::: Dependencies installed successfully :::::::::::::\n"
else
    echo -e "\n ::::::::::::: Dependencies installation failed :::::::::::::\n"
    exit 1
fi

echo -e "\n<==========Preparing for the project to run================>\n"
# Run the frontend in the background
echo -e "\n<==========...Starting the frontend...===============>\n"
cd frontend || exit

# install trunk if it is not installed
if ! cargo install trunk; then
    echo -e "\n<==========Installing trunk...===============>\n"
    cargo install trunk
fi

trunk serve &
FRONTEND_PID=$!

echo -e "\nFrontend running on http://127.0.0.1:8080\n"
# Wait a moment for the frontend to start
sleep 2


# Run the API in the background
echo -e "\n<==========...Starting the API...===============>\n"

#start mongodb  if it is installed
if brew services list | grep -q "mongodb-community"; then
    brew services start mongodb-community
else
    echo -e "\n ::::::::::::: MongoDB is not installed :::::::::::::\n"
    echo -e " ::::::::::::: Installing MongoDB :::::::::::::\n"
    brew install mongodb-community
    brew services start mongodb-community
fi
# cd back into the api directory
cd ../api || exit
# build the api
cargo build 
cargo run --bin api &
# get the pid of the api
API_PID=$!




# Handle script termination
trap 'kill $FRONTEND_PID $API_PID' EXIT

# Wait for both processes
wait $FRONTEND_PID $API_PID
echo " ::::::::::::: API running on http://127.0.0.1:8000 :::::::::::::"