pos = 20;
popSize = 300;

cutted = dump_pop((pos.*popSize+1):(pos+1).*popSize, :);
X = cutted(:, 2);
Y = cutted(:, 3);
Z = cutted(:, 4);

[XX, YY] = meshgrid(-1:0.1:1, -1:0.1:1);
ZZ = XX.^2 + (YY-1).^2;

CC = YY - XX.^2;

subplot(1, 2, 1)
scatter3(X, Y, Z, "x", "markeredgecolor", "red")
hold on
surf(XX, YY, ZZ)
surf(XX, YY, CC)
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

