
include <iostream>

class Player {
    public:
        int puntos;
};

int main() {

    Player *pepe = new Player;

    std::cout<<pepe->puntos;

    return 0;
}
