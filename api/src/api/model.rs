use image::{GrayImage, Luma, open};
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::ffi::c_str;

pub fn classify(input_image: &str, output_image: &str,labels:&str) -> PyResult<()> {

    pyo3::prepare_freethreaded_python();
    let _ = Python::with_gil(|py| -> PyResult<()> {
        // Define the Python script as a string
        let python_code = PyModule::from_code(
            py,
            c_str!("def detect_and_draw(input_image_path, output_image_path, labels):
            import cv2
            import numpy as np
            import tensorflow as tf

            print('....................')
            print(f'labels  {labels}')
            filtered_class_names = {}
            class_names = {
            1: 'person', 2: 'bicycle', 3: 'car', 4: 'motorcycle', 5: 'airplane',
            6: 'bus', 7: 'train', 8: 'truck', 9: 'boat', 10: 'traffic light',
            11: 'fire hydrant', 13: 'stop sign', 14: 'parking meter', 15: 'bench',
            16: 'bird', 17: 'cat', 18: 'dog', 19: 'horse', 20: 'sheep',
            21: 'cow', 22: 'elephant', 23: 'bear', 24: 'zebra', 25: 'giraffe',
            27: 'backpack', 28: 'umbrella', 31: 'handbag', 32: 'tie', 33: 'suitcase',
            34: 'frisbee', 35: 'skis', 36: 'snowboard', 37: 'sports ball', 38: 'kite',
            39: 'baseball bat', 40: 'baseball glove', 41: 'skateboard', 42: 'surfboard',
            43: 'tennis racket', 44: 'bottle', 46: 'wine glass', 47: 'cup', 48: 'fork',
            49: 'knife', 50: 'spoon', 51: 'bowl', 52: 'banana', 53: 'apple',
            54: 'sandwich', 55: 'orange', 56: 'broccoli', 57: 'carrot', 58: 'hot dog',
            59: 'pizza', 60: 'donut', 61: 'cake', 62: 'chair', 63: 'couch',
            64: 'potted plant', 65: 'bed', 67: 'dining table', 70: 'toilet',
            72: 'tv', 73: 'laptop', 74: 'mouse', 75: 'remote', 76: 'keyboard',
            77: 'cell phone', 78: 'microwave', 79: 'oven', 80: 'toaster', 81: 'sink',
            82: 'refrigerator', 84: 'book', 85: 'clock', 86: 'vase', 87: 'scissors',
            88: 'teddy bear', 89: 'hair drier', 90: 'toothbrush'   
            }
            
            if labels != 'None':
                sub_labels = labels.split(',')
                for key, value in class_names.items():
                    if value in sub_labels:
                        filtered_class_names[key] = value

        
            model_path = '../model/frozen_inference_graph.pb'
            print('....................')
            print('Loading model...', model_path)
            print('....................')
            detection_graph = tf.Graph()
            with detection_graph.as_default():
                od_graph_def = tf.compat.v1.GraphDef()
                with tf.compat.v2.io.gfile.GFile(model_path, 'rb') as fid:
                    serialized_graph = fid.read()
                    od_graph_def.ParseFromString(serialized_graph)
                    tf.compat.v1.import_graph_def(od_graph_def, name='')
        
        
            
            image_np = cv2.imread(input_image_path)
            with detection_graph.as_default():
                with tf.compat.v1.Session(graph=detection_graph) as sess:
                    # Get input and output tensors
                    image_tensor = detection_graph.get_tensor_by_name('image_tensor:0')
                    detection_boxes = detection_graph.get_tensor_by_name('detection_boxes:0')
                    detection_scores = detection_graph.get_tensor_by_name('detection_scores:0')
                    detection_classes = detection_graph.get_tensor_by_name('detection_classes:0')
                    num_detections = detection_graph.get_tensor_by_name('num_detections:0')
        
                    (boxes, scores, classes, num) = sess.run(
                        [detection_boxes, detection_scores, detection_classes, num_detections],
                        feed_dict={image_tensor: np.expand_dims(image_np, axis=0)})
        
                    boxes = np.squeeze(boxes)
                    scores = np.squeeze(scores)
                    classes = np.squeeze(classes).astype(np.int32)
        
                    # Draw bounding boxes
                    height, width, _ = image_np.shape
                    for i in range(len(boxes)):
                        if scores[i] > 0.5:  # Adjust threshold as needed
                            y_min, x_min, y_max, x_max = boxes[i]
                            x_min, x_max = int(x_min * width), int(x_max * width)
                            y_min, y_max = int(y_min * height), int(y_max * height)
                            if labels == 'None':
                                class_name = class_names.get(classes[i], 'Unknown')
                            else:
                                class_name = filtered_class_names.get(classes[i], 'Unknown')
                            if class_name == 'Unknown':
                                continue
                            cv2.rectangle(image_np, (x_min, y_min), (x_max, y_max), (0, 255, 0), 2)
                            label = f'{class_name}: {scores[i]:.2f}'
                            font_scale = 0.5
                            font_thickness = 1
                            text_size, _ = cv2.getTextSize(label, cv2.FONT_HERSHEY_SIMPLEX, font_scale, font_thickness)
                            text_width, text_height = text_size
                            label_background_color = (0, 255, 0)
                            cv2.rectangle(image_np, (x_min, y_min - text_height-10), (x_min + text_width, y_min), label_background_color, -1)
                            text_color = (0,0,0)
                            cv2.putText(image_np, label, (x_min, y_min - 10),
                                cv2.FONT_HERSHEY_SIMPLEX, font_scale, text_color, font_thickness)
                    cv2.imwrite(output_image_path, image_np)
                    return output_image_path"),
                    c_str!(""),
                    c_str!(""),
        )?;

        

        let detect_and_draw = python_code.getattr("detect_and_draw")?;
        println!("Python function 'detect_and_draw' retrieved successfully.{}",detect_and_draw);


        // Call the function with Rust tuple of positional arguments
        let args = (input_image, output_image, labels);
        
        let result = detect_and_draw.call1(args);
        
        
        match result {
            Ok(output) => println!("Output {:?}", output),
            Err(e) => println!("Failed: {:?}", e),
        }

        Ok(())
    });

    Ok(())
}
