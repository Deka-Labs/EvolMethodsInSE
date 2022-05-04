

par_start = 0;
par_end = 1;
par_step = 0.05;

par = par_start:par_step:par_end;
par_name = "Mutation chance"

Time = dump_pop_err_mutation(:, 2);
Errors = dump_pop_err_mutation(:, 3);

subplot(1, 2, 1);
plot(par, Time);
title("Solve time");
xlabel(par_name);
ylabel("time (s)");


subplot(1, 2, 2);
plot(par, Errors);
title("Avg error");
xlabel(par_name);
ylabel("E");
