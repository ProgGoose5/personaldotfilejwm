#!/usr/bin/env bash
# instalar-jwm-ligero.sh
# Instala JWM + dependencias mínimas y coloca un .jwmrc ligero en antiX/Debian.
# Uso: bash instalar-jwm-ligero.sh
cat > /home/goosely/Desktop/goosely/Github_Repos/personaldotfilejwm/rofitheme/left_down_corner.rasi << 'EOF'
configuration {
    location: 6; /* Bottom-left corner */
    width: 30%; /* Adjust width as needed */
}

* {
    background: #1a1a1a; /* Gray background */
    foreground: #a1a1a1; /* Barely white text */
    selected-background: #1d1d1d; /* Slightly lighter gray for selected items */
    selected-foreground: #FFFFFF; /* White text for selected items */
    border: 0px;
    padding: 5px;
    font: "monospace 12";
}
EOF