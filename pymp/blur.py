import numpy as np
from numbapro import cuda
from PIL import Image

@cuda.jit("void(float32[:], float32[:])", target="gpu")
def blur(input_img, blurred_img):
  index = cuda.grid(1)
  if(index >= input_img.shape[0]):
    return
  blurred_img[index] = 4.0


if __name__ == "__main__":
  img = np.ones(100)
  blurred_img = np.zeros(100)
  d_input_img = cuda.to_device(img)
  d_blurred_img = cuda.device_array(img.shape[0])

  threads_per_block = 256
  n_blocks = (img.shape[0] + threads_per_block-1) / threads_per_block

  for num in blurred_img:
    print num
  blur[n_blocks,threads_per_block](d_input_img, d_blurred_img)

  print("####################")

  d_blurred_img.copy_to_host(blurred_img)
  for num in blurred_img:
    print num
  print("Finished")
