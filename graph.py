xs=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]
qs=[2.896, 3.725, 4.665, 5.766, 7.147, 9.01, 11.779, 16.598, 28.142, 115.057]
mean=[0.005584239739739734, 0.012454494816627517, 0.021477883175049863, 0.03328743478837819, 0.05002712180667627, 0.07497519001553535, 0.11629710769647908, 0.19977120909336638, 0.4409246391966978, 4.635270525304119]
p25=[0.005000000000000348, 0.011749999999999748, 0.02060000000000033, 0.032249999999999966, 0.04848969793958819, 0.07254999999999961, 0.11186222666857239, 0.1899249999999937, 0.4102555555555478, 3.6066935890964267]
p75=[0.006100000000000106, 0.013100000000000146, 0.02230000000000032, 0.03429999999999976, 0.05143028605721161, 0.07718333333333331, 0.12010861797913444, 0.20782597824727372, 0.4662440217995723, 5.383877755511032]
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

