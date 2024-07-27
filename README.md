# Individual Neurons Simulation

This project simulates the membrane potential (V_m) based on excitatory conductance (g_e) in individual neurons. The simulation is grounded in neurophysiological principles and aims to provide a computational model of neuronal behavior.

## Overview

Neurons are complex electrochemical systems that can be modeled computationally to understand their behavior under various conditions. This project is based on concepts from neurophysiology, particularly focusing on the membrane potential (V_m) and its relationship with excitatory inputs.

## Key Concepts

### Membrane Potential (V_m)
- **Membrane Potential**: The difference in charge across the neuronal membrane. It is a critical aspect of neuronal function, influencing the ability of neurons to fire and communicate with each other.
- **Equilibrium Membrane Potential**: With constant inputs, the membrane potential eventually stabilizes into a steady state where the net current (I_net) is zero. This equilibrium can be determined by solving the net current equation.

### Conductances (g_e)
- **Excitatory Conductance (g_e)**: Represents the conductance of excitatory ion channels, such as those allowing Na+ ions to enter the neuron. The value of g_e affects the membrane potential and the likelihood of the neuron reaching the threshold to fire an action potential.
- **Leak Conductance (g_leak)**: Represents the conductance of leak channels, which are always open and help maintain the resting membrane potential.

### Action Potential
- **Threshold**: The membrane potential at which a neuron fires an action potential. Once this threshold is crossed, an electrical pulse is sent down the axon, and the neuron resets its membrane potential.

### Equations
- **Net Current (I_net)**: The sum of currents through different channels, each influenced by their respective conductances and equilibrium potentials.

$$I_{net}(t) = g_e(t)\bar{g_e}(V_m(t)-E_e)+g_i(t)\bar{g_i}(V_m(t)-E_i)+g_l(t)\bar{g_l}(V_m(t)-E
_l)$$

- **Membrane Potential Update**:
$$V_m(t+1)=V_m(t)+dt_{vm}[g_e(t)\bar{g_e}(E_e-V_m(t))+g_i(t)\bar{g_i}(E_i-V_m(t))+g_l(t)\bar{g_l}(E_l-V_m(t))]$$
