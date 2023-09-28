import os
from ppsim import Simulation
import matplotlib.pyplot as plt

import timeit



def finished(a):
    return len(a) == 1
    

def one_opinion(config):
    unique_opinions = set(state.opinion for state in
    config.keys())
    return len(unique_opinions) == 1

def undecided_state_dynamics(initiator, responder):
    if initiator == "Undecided" and responder != "Undecided" : 
        return responder, responder
    elif initiator != responder and responder != "Undecided":
        return "Undecided" , responder
    
def even_distributed(agents, opinions):
    agents_per_opinion = round(agents /opinions)
    dict = {"Undecided":0}
    for i in range(1,opinions+1):
        dict[i] = agents_per_opinion
    return dict

start = timeit.default_timer()
n = 10 ** 7
x = even_distributed(n,1000)
#init_config = {"Undecided":0 , 1: 300, 2: 100, 3: 100, 4: 100, 5: 100, 6: 100, 7: 100, 8: 100, 9: 100, 10: 100, 11: 100, 12: 100, 13: 100, 14: 100, 15: 100, 16: 100,17: 100, 18: 100, 19: 100, 20: 100, 21: 100, 22: 100}
#init_config = {"Undecided":0 , 1: 200, 2: 100,3:100,4:100,5:100}
#sim = Simulation(even_distributed(n,100),undecided_state_dynamics,simulator_method="Sequential")
sim = Simulation(x,undecided_state_dynamics)
# sim = Simulation(init_config,undecided_state_dynamics)
#print(n ,sim.state_list)
sim.run(run_until=20)



#p = sim.history.plot()
#plt.savefig("test.png")

stop = timeit.default_timer()

print('Time: ', (stop - start)*1000) 
#print(sim.history.tail(5))

# print(sim.history.tail(1))
# test = True
# while test:
#     sim = Simulation(init_config,undecided_state_dynamics)
#     sim.run(run_until=finished)
#     if(sim.history.tail(1).to_dict(orient='records')[0]['1']==0):
#         test = False
    
#     print(sim.history.tail(1).to_dict(orient='records')[0]['1'])

