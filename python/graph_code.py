import matplotlib.pyplot as plt
import numpy as np

N = len(mean)

fig, ax = plt.subplots()

xaxis=np.arange(1, N+1)
ax.bar(xaxis, qs, color='r')
plt.savefig('img/rust-qs.png')

fig, ax = plt.subplots()
ax.bar(xaxis, mean, color='r', yerr=np.vstack([p25, p75]), capsize=8)
plt.savefig('img/rust-qd.png')

fig, ax = plt.subplots()
ax.bar(xaxis, mean, color='r')
plt.savefig('img/rust-qd-no-bar.png')
