import cv2

image = cv2.imread('./uploads/classified.png')
gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
cv2.imshow('Gray image', gray)
cv2.waitKey(0)
cv2.destroyAllWindows()

 if scores[i] > 0.5:  # Adjust threshold as needed
    y_min, x_min, y_max, x_max = boxes[i]
    x_min, x_max = int(x_min * width), int(x_max * width)
    y_min, y_max = int(y_min * height), int(y_max * height)
    class_name = class_names.get(classes[i], 'Unknown')
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