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

*Our timeline*

*Front end(November 13th just in time to show for the project update)*
- Design the interface of the website. What it will look like, what options we give the user.
- Give the website access to the camera

*Back end working simultaneously with front end(November 30th)*

- Install Open cv
- Build and train an Image Classifier to identify objects(November 20)
- Incorporate the model into the website with an API (December 7th)
- Write our presentation and final report (December 10th)

*Done* : ) 







