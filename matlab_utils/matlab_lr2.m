pos = 50;
popSize = 300;

cutted = dump_pop((pos.*popSize+1):(pos+1).*popSize, :);
X_1 = cutted(:, 2)
X_2 = cutted(:, 3)
X = cutted(:, 4);
Y = cutted(:, 5);
Z = cutted(:, 6);

[XX, YY] = meshgrid(-4:0.1:4, -4:0.1:4);
F1 = (XX.^2/2)+(YY+1).^2/13+3
F2 = (XX.^2/2)+(2*YY+2).^2/15+1
F3 = (XX+2*YY-1).^2/175+(2*YY-XX).^2/27-13

subplot(1, 4, 1)
scatter3(X, Y, Z, "x", "markeredgecolor", "red")
hold on

xlabel("F1")
ylabel("F2")
zlabel("F3")
title("Full 3D view")
hold off

subplot(1, 4, 2)
scatter3(X_1, X_2, X, "x", "markeredgecolor", "red")
hold on
surf(XX, YY, F1)

xlabel("X")
ylabel("Y")
zlabel("F1")
title("Criterion 1")
hold off

subplot(1, 4, 3)
scatter3(X_1, X_2, Y, "x", "markeredgecolor", "red")
hold on
surf(XX, YY, F2)

xlabel("X")
ylabel("Y")
zlabel("F2")
title("Criterion 2")
hold off

subplot(1, 4, 4)
scatter3(X_1, X_2, Z, "x", "markeredgecolor", "red")
hold on
surf(XX, YY, F3)

xlabel("X")
ylabel("Y")
zlabel("F3")
title("Criterion 3")
hold off

