pos = 30;
popSize = 300;

cutted = dump_pop((pos.*popSize+1):(pos+1).*popSize, :);
X_1 = cutted(:, 2);
X_2 = cutted(:, 3);
X = cutted(:, 4);
Y = cutted(:, 5);
Z = cutted(:, 6);

for i=size(cutted, 1):-1:1
  for j=size(cutted, 1):-1:1
    if (X(i) > X(j) && Y(i) > Y(j) && Z(i) > Z(j))
      cutted(i, :) = [];
      break
    end
  end
end

X_1 = cutted(:, 2);
X_2 = cutted(:, 3);
X = cutted(:, 4);
Y = cutted(:, 5);
Z = cutted(:, 6);

[XX_1, XX_2] = meshgrid(-4:0.1:4, -4:0.1:4);

F1 = (XX_1.^2/2)+(XX_2+1).^2/13+3;
F2 = (XX_1.^2/2)+(2*XX_2+2).^2/15+1;
F3 = (XX_1+2*XX_2-1).^2/175+(2*XX_2-XX_1).^2/27-13;


scatter3(X, Y, Z, "x", "markeredgecolor", "red")

xgr = linspace(min(X), max(X), 100);
ygr = linspace(min(Y), max(Y), 100);
[XXgr, YYgr] = meshgrid(xgr, ygr);
ZZgr =  griddata(X, Y, Z, XXgr, YYgr);
hold on
mesh(XXgr, YYgr, ZZgr);


xlabel("F1")
ylabel("F2")
zlabel("F3")
title("Full 3D view")
shading interp
hold off

