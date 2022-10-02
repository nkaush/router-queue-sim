import matplotlib.pyplot as plt
import numpy as np

fig, ax = plt.subplots()

p25 = [m - p for m, p in zip(mean, p25)]
p75 = [p - m for m, p in zip(mean, p75)]
xaxis = np.arange(1, 11)

ax.bar(xaxis, qs, color='r')
ax.set_xticks(xaxis)
ax.set_xticklabels(xs)
plt.xlabel("Arrival Rate / Service Rate (A/R)")
plt.ylabel("Mean Maximum Queue Size")
plt.title(f"Mean of Max Queue Size for N={N} Simulations Per A/R")
plt.savefig('results/rust-qs.png')

fig, ax = plt.subplots()
ax.bar(xaxis, mean, color='r', yerr=np.vstack([p25, p75]), capsize=8)
ax.set_xticks(xaxis)
ax.set_xticklabels(xs)
plt.xlabel("Arrival Rate / Service Rate (A/R)")
plt.ylabel("Mean of Mean Packet Queueing Delay by Simulation (Seconds)")
plt.title(f"Mean of Mean Queuing Delay for N={N} Simulations Per A/R")
plt.savefig('results/rust-qd.png')

fig, ax = plt.subplots()
ax.bar(xaxis, mean, color='r')
ax.set_xticks(xaxis)
ax.set_xticklabels(xs)
plt.xlabel("Arrival Rate / Service Rate (A/R)")
plt.ylabel("Mean of Mean Packet Queueing Delay by Simulation (Seconds)")
plt.title(f"Mean of Mean Queuing Delay for N={N} Simulations Per A/R")
plt.savefig('results/rust-qd-no-bar.png')
