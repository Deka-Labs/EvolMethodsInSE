pos = 100;
popSize = 300;

cutted = dump_pop((pos.*popSize+1):(pos+1).*popSize, :);
X = cutted(:, 2);
Y = cutted(:, 3);
Z = cutted(:, 4);

[XX, YY] = meshgrid(-4:0.1:4, -4:0.1:4);
ZZ = 2500 - (XX.^2 + YY - 11).^2 - (XX + YY.^2 - 7).^2;

subplot(1, 2, 1)
scatter3(X, Y, Z, "x", "markeredgecolor", "red")
hold on
surf(XX, YY, ZZ)
xlabel("X")
ylabel("Y")
zlabel("Z")
hold off

subplot(1, 2, 2)
scatter3(X, Y, Z, "x", "markeredgecolor", "red")
hold on
surf(XX, YY, ZZ)
xlabel("X")
ylabel("Y")
zlabel("Z")
hold off
view(2)

