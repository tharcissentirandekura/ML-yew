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
