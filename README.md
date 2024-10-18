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







