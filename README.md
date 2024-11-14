# groupproject

Team: Reefayat, Jesse, Tharcisse, Kelig

**Project Proposal**

We were going to make a website that incorporates machine learning to: classify images in the center of a frame. The front end is going to look like a camera or a picture then the AI on the back end is going to alter the picture, classify the object in the middle of the frame, and send it back to the front end. Then the front end displays the altered picture and defines the classified object. 

**How are you doing it?**

*Rust and WebAssembly*
- Use web assembly and rust to create an interactive front end
- Use rust frameworks like “yew” to create the front-end components
- Create an API using tools like Actix Web to be able to host our model and process requests on the website

*Image classifier with OpenCV*

- Use the opencv crate on rust to get access to all the vision capabilities
- Use a pre-trained model like ResNet
- Integrate model using the tch crate
- Process requests on the front end through API endpoints


**What do you need to learn to do it?**

- Learn how Rust compiles into WebAssembly 
- Learn how to use frameworks like yew and actix
- Learn how to use the opencv crate for image processing
- Learn the basics of how the image classification models work
- Learn how to load and use pre-trained models in Rust
- Learn how to develop an API with proper end-points
- Learn how to integrate AI models into Rust.


**A proposed schedule with milestones (the status update will discuss the milestones so you might want to track these on GitHub.)**

We plan on having two groups one coding the front end(the website) one coding the vision model then our last step will be connecting them. So we shall have two timelines, one for the front end and one for the back end.

**Our timeline**

**Front end**
*Starting after break and until Nov 13th Status update*
- Design the user interface for website, defining what options the user will have.
- Begin implementing the front-end using your chosen framework.
- Add basic elements to capture user inputs and access the camera.
*After Nov 13th Status update*
- Complete the camera integration, allowing the website to capture images or video from the user's camera.
- Work on front-end functionality, such as buttons for capturing images and sending them to the backend for processing.
- Set up the backend web API using Actix
- Build initial endpoints for the backend that will eventually handle the classification requests.
- debug and refine!!!

**Back end working simultaneously with front end**
*Starting after break until Nov 13th Status update*
- Install OpenCV and learn to manipulate images and video.
- Learn basic OpenCV operations 
- Start experimenting with model.
- Continue refining the front-end and ensure that images captured via the camera are properly sent to the backend.
*After Nov 13th Status Update*
- Figure out what ML framework to use
- Ensure that the pre-trained/trained model can process images and return predictions.
- Test the model locally to confirm it works with input images/video feed
- Integrate the trained AI model into the backend API.
- Build final API route that accepts the camera image from the front-end and returns classification results.
- Refine and think of more features!!

*Done* : ) 

---------------------------------------- x -----------------------------------------------------------------------------

**Status Update** Nov. 15

*Summary of Project*

We are going to make a website that utilizes OpenCV to classify discernible objects in an image. We expect the front-end to be a WebAssembly, Yew, and ActixWeb project that allows a user to upload an image which is then passed in through a RestAPI which then processes the image and passes it to a pre-trained model using Rust’s OpenCV and Image crates, and returns an image with bounding boxes around objects along with labels for the objects. 

*Changes since initial idea*:

We decided not to proceed with our idea of real-time object classification, as we were having complications figuring out how to integrate OpenCV’s camera handling methods with the front end client. So we decided to use images which are then just saved in a database/local storage.

*Progress for Website ( Backend with Actix and frontend with yew and web assembly to compile rust to browser readable code)*:
To support the image upload and retrieval process, we defined key routes in the Actix project. These routes establish the mechanisms for sending, processing, and retrieving images, effectively connecting the frontend with our machine learning model on the backend. 
Home Page Route: The home page route serves as the initial landing point for users accessing the API. Although its primary function is introductory, it helps verify that the API is running and accessible. 
Upload Image Route: The upload image route is a POST endpoint that allows the frontend client to send images to the backend. When a user uploads an image on the frontend, it is transmitted to this route, where the backend can then process it. Once the image reaches the backend, it is forwarded to our machine learning model, which performs the required analysis or transformations. This route is central to the functionality of our application, as it triggers the backend’s core task of image processing. We are almost done with this right by now, but we are still thinking the best way to store the image we get from the user.
Get Processed Image Route: The get image route is a GET endpoint that the frontend can call to retrieve the processed image. Once the machine learning model completes processing the uploaded image, this route enables the frontend to request the processed image and display it to the user. The get image route finalizes the interaction between the frontend and backend, delivering the output of our machine learning pipeline back to the client for user viewing.

*Progress for Image Processing and Classification (OpenCV and Image)*:

So far we have figured out how to use OpenCV and Image crates to create a program that lets you specify a path to an image, and returns the same image with green bounding boxes around discernible objects in the image.
The next steps include figuring out what pre-trained model to use, and how we plan to pass in images to the model.

*Revised Schedule*:

*Front end* 

Finish designing the website's user interface, defining the user's options.
Add basic elements to capture user inputs and images.
Work on front-end functionality, such as buttons for capturing images and sending them to the backend for processing.
Debug and refine!!!
Integration with Back-End API
Image data and data handling

*Back end*

Start experimenting with the model.
We plan on using a PyTorch model and incorporating it into our backend using an unsafe block
Continue refining the front-end and ensure that images captured via the camera are properly sent to the backend. 
Ensure that the pre-trained/trained model can process images and return predictions.
Test the model locally to confirm it works with input images
Integrate the trained AI model into the backend API.
Build a final API route that accepts the image from the front-end and returns classification results.
Refine and think of more features!!


*Conclusion*:
We are on track to finish by the due date, although we have been facing our various problems none seem so insurmountable that we will not be able to finish by the due date. We still plan on having the image processing part of the project done by the 20th and now we understand that the goal to have the website done by the progress report date was a little overly ambitious but we are mostly done, hopefully that too will be done by the 20th and we will spend the rest of the lab combining them.
















