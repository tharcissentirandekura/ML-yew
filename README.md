https://github.com/brooks-builds/full-stack-todo-rust-course/tree/main/backend


- There are things we need to know for our project 
- I think since we are using an api to be able to hook backend data to frontend
        : - We need a simple database storage 
         - We can use  Mongodb database
         - How it works:
            - create a database
            - connect to it when running the backend
            - when uploading an image:
                - save the image to the database
            - Then we can get that image easily when sending it to our machine learning model
- I defined the database connection
        /src/api/db.rs

then in 
        src/main.rs : I connected to the database connection by called connect_db

- In the file 
        src/upload.rs :
    - we added get_image handler for  /image
    - I added upload_image handler for /upload route
        - the upload_image handlers:
                - Have a struct for Image with id,name, and data which is vec<u8> for image
                - struct UploadImage for fields to be uploaded
                - a handler/ helper function for saving image to database
                - 



// Check those files and see if you are getting what is happening.

- to run it 

navigate to 
    src/api/
    - then : cargo watch -x run -p api
    - That allow us to auto refresh our changes





# For frontend 
- we will use yew and you can check the documentation for reference to create buttons, links, images display, onclick, onsubmit,...
- For styling our website so that it looks cool and amazing:
        - we will use tailwind css : We can just look up the styles we want to use from its website
- For the look of frontend 
        - we can come up with a design that is cool.

# make a beatiful markdown

# Running the Project

To run the entire project, you can use the `./run-all.sh` script. This script will handle starting both the backend and frontend components of the application. Follow the steps below to get started:

## Prerequisites

Ensure you have the following installed on your system:
- Rust and Cargo
- MongoDB

## Steps to Run

1. **Clone the Repository**

   First, clone the repository to your local machine:

   ```bash
   git clone https://github.com/brooks-builds/full-stack-todo-rust-course.git
   cd full-stack-todo-rust-course
   ```

2. **Set Up the Backend**

   Navigate to the backend directory and ensure all dependencies are installed:

   ```bash
   cd backend
   cargo build
   ```

3. **Run the Project**

   Navigate back to the root directory and execute the `run-all.sh` script:

   ```bash
   cd ..
   ./run-all.sh
   ```

   This script will start both the backend and frontend servers.

4. **Access the Application**

   Once the servers are running, you can access the application in your client web browser at `http://127.0.0.1:8000/`.

## Troubleshooting

- If you encounter any issues, ensure that MongoDB is running and accessible.
- Check the logs for any error messages and ensure all dependencies are correctly installed.


