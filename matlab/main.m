% layer 1: inputs -> 3 nodes and a bias term
inputs = [1;
          1;
          1;
          1];

% layer 2: middle layer
numnodes = 2;
weights  = ones(numnodes, size(inputs, 1)); % holds weights and bias values
middle   = activation_func(weights * inputs);

% layer 3: output
numnodes = 1;
weights  = ones(numnodes, size(middle, 1));
output   = activation_func(weights * middle);
