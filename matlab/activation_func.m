function out = activation_func(a)
  if a > 0
    out = a;
  else
    out = 0.1 * a;
  end
end
