# Quantum Rust: An Interactive, Performant Quantum Simulator

# Group Name: Unitary Evolution 

# Group Members: 
Shrikar Dulam (sdulam2), Bobby Mandell (bobbym3), Aashima Sisodia (aashima2) 

# Introduction
We seek to build an interactive quantum computing simulator that is both accessible and effective. The primary objective is to design an application that allows users to construct and run quantum circuits, visualize quantum states and measurement results, and explore quantum noise and error-correction concepts. We also hope to implement multiple approaches, from straightforward statevectors to advanced tensor-network techniques relevant to current research.

We chose this project due to our shared backgrounds as CS + Physics majors. Based on quantum computing’s rapid development, we feel that developing this simulator would strengthen our understanding of quantum and computational physics and aid us in future work related to this field. Additionally, the project lets us explore Rust’s safety, performance, and concurrency features in a practical setting.

# Technical Overview
Starting with the frontend, we hope to make an interactive GUI using libraries like Iced, Slint, Yew, and WASM. We would like to visualize quantum circuits graphically by displaying the different gates and qubit wires. Optionally, we would like to do something similar to IBM’s composter and have the user be able to drag and drop circuits into place. We would also like to display statistical results after running the circuits using a library like Chart.JS.  The primary result would be to show the state vector distributions given the final state probabilities, possibly even dynamically sampling the distribution to create shot-by-shot animations. Other meaningful metrics would be expectation values for a given observable and backend performance times for running the circuit across different implementations. 
For the backend, we would start off by implementing the standard statevector representation, where we start with an initial state and evolve it by applying gates on each qubit. We hope to implement a comprehensive gateset including gates like the X, Y, Z, Hadamard, arbitrary rotation gates with specific rotations like S and T, and a measurement operator to collapse the quantum state into classical bits. We will also implement multi-qubit gates like the control and Toffolli gates. We will try to parallelize the computations to improve performance. 
To expand upon this project, we can also increase the computational burden and work with full density matrices to model quantum error correction. In this case, we would apply noise through Kraus operators, showcasing depolarization, bit-flip, phase-flip, and amplitude dampening errors. Another extension would be to implement a tensor-based approach using matrix-product states. In this case, we have to recompile our circuit gates into a usable tensor and apply tensor operations like reshaping and contracting to run the circuit. We hope to optimize the operations for better performance and scalability.

With these methods, we can try running quantum algorithms on a small enough scale. For example, we can try factoring small numbers with Shor’s algorithm, run Grover search, and/or write a script to generate circuits that simulate a given Hamiltonian via Trotterization. 

For our first checkpoint, we hope to implement the statevector backend and finalize frontend details. For our second checkpoint, we hope to have implemented a usable frontend and some of the simpler extensions like using density matrices or running a quantum algorithm. 

We foresee implementing the frontend to be one of the bigger challenges, as we are inexperienced with web development and will have to navigate using external libraries, properly formatting the display, and handling user inputs. Another challenge would be in making sure we organize our code well, as we have a lot of distinct components that should all link together seamlessly. We need to ensure we practice good modular design and maintain the codebase. Lastly, optimizing the performance of our code will be one of the final hurdles. After all, we are trying to classically simulate a quantum computer which grows exponentially in scale, so making sure we parallelize our code and make it as efficient as possible will be one of our overarching goals. 

# Inspiration 
While this project is a bit technical due to its quantum nature, we have found plenty of useful resources that can help assist us. 


UIUC itself offers a computational physics class, and one of the projects is simulating a quantum computer in Python/C++. There is a comprehensive guide on the project discussing various ways to approach a naive backend linked below. <br>  
https://courses.physics.illinois.edu/PHYS446/sp2023/QC/Overview.html 

There are also some resources for backend extensions: <br>  https://iopscience.iop.org/article/10.1088/2058-9565/ab5887 (QEC/density matrix) <br> https://pennylane.ai/qml/demos/tutorial_mps (Tensor methods)

For the frontend, we found a few applications that inspired us as well:  <br> https://quantum.cloud.ibm.com/composer <br> https://algassert.com/quirk



