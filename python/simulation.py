from tqdm import tqdm
import numpy as np

GRANULARITY = 0.1
GRANULARITY_INV = 10

class Time:
    def __init__(self, asint):
        self.asint = asint
        self.asfloat = asint / GRANULARITY_INV

    def __hash__(self):
        return hash(self.asint)

    def __eq__(self, other):
        return self.asint == other.asint

class Packet:
    def __init__(self, it: Time):
        self.insertion_time = it.asfloat
        self.removal_time = None

    def sent(self, t: Time):
        self.removal_time = t.asfloat
    
    def get_queueing_delay(self):
        return self.removal_time - self.insertion_time

class RouterQueue: 
    def __init__(self):
        self.queue = []
        self.queueing_delays = []

    def pop(self, t):
        if self.queue:
            packet = self.queue.pop(0)
            packet.sent(t)
            self.queueing_delays.append(packet.get_queueing_delay())
            return packet
        else:
            return None
    
    def push(self, packet):
        self.queue.append(packet)

class SingleSimulation:
    def __init__(self, A, T=1000):
        self.A = A
        self.T = T
        self.reset()

    def reset(self):
        self.queue = RouterQueue()
        self.packet_arrivals = self.generate_times()

    def simulate(self):
        num_packets_pulled = 0
        num_packets_pushed = 0
        max_queue_size = 0
        
        for t in [Time(i) for i in range(0, int(self.T * GRANULARITY_INV))]:
            for _ in range(self.packet_arrivals.get(t, 0)):
                self.queue.push(Packet(t))
                num_packets_pushed += 1

            max_queue_size = max(max_queue_size, len(self.queue.queue))

            if self.queue.pop(t) is not None:
                num_packets_pulled += 1
 
        assert num_packets_pushed == self.A * self.T
        return sum(self.queue.queueing_delays) / len(self.queue.queueing_delays), max_queue_size

    def generate_times(self):
        num_packets = self.A * self.T
        multiplier = self.T * GRANULARITY_INV

        times = (np.random.random(num_packets) * multiplier).astype(int)

        num_queued = {}
        for t in [Time(t) for t in times]:
            num_queued[t] = num_queued.get(t, 0) + 1
        
        return num_queued

class Simulation:
    def __init__(self, A, N=1000):
        self.simulations = [SingleSimulation(A) for _ in range(N)]
        self.avg_qds = []
        self.max_qs = []

    def run(self):
        for s in tqdm(self.simulations, disable=False):
            qd, qs = s.simulate()
            self.avg_qds.append(qd)
            self.max_qs.append(qs)

p25 = []
p75 = []
mean = []
qs = []
for A in range(1, 11):
    print(f"Simulating for A={A}")
    s = Simulation(A)
    s.run()
    p25.append(np.percentile(s.avg_qds, 25))
    p75.append(np.percentile(s.avg_qds, 75))
    mean.append(np.mean(s.avg_qds))
    qs.append(np.mean(s.max_qs))

print(f"p25={p25}")
print(f"p75={p75}")
print(f"mean={mean}")
print(f"qs={qs}")