import numpy as np
from timeit import default_timer as timer
from numbapro import vectorize

def vectorAdd(a, b, c):
  for i in xrange(a.size):
    c[i] = a[i] + b[i]

@vectorize(["float32(float32, float32)"], target='gpu')
def vectorAddGPU(a, b):
  return a+b

def main():
  # CPU single thread
  N = 32000000
  A = np.ones(N, dtype=np.float32)
  B = np.ones(N, dtype=np.float32)
  C = np.zeros(N, dtype=np.float32)

  start_t = timer()
  vectorAdd(A, B, C)
  vectorAdd_t = timer() - start_t
  print("C[:5] = " + str(C[:5]))
  print("C[-5:] = " + str(C[-5:]))
  print("Time (CPU): %f seconds" % (vectorAdd_t))

  #GPU accelerated
  A = np.ones(N, dtype=np.float32)
  B = np.ones(N, dtype=np.float32)
  C = np.zeros(N, dtype=np.float32)

  start_t = timer()
  C = vectorAddGPU(A, B)
  vectorAdd_t = timer() - start_t
  print("C[:5] = " + str(C[:5]))
  print("C[-5:] = " + str(C[-5:]))
  print("Time (GPU): %f seconds" % (vectorAdd_t))

if __name__ == "__main__":
  main()
