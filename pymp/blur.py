import numpy as np
from numbapro import cuda
from PIL import Image

@cuda.jit("void(float32)", target="gpu")
def blur(img):
  return


if __name__ == "__main__":
  print("asd")
