import sys
import pytesseract
from PIL import Image

def extract_text_from_image(image_path):
    with Image.open(image_path) as img:
        img = img.convert("L")
        text = pytesseract.image_to_string(img)
        return text

if __name__ == '__main__':
    image_path = sys.argv[1]
    text = extract_text_from_image(image_path)
    print(text)