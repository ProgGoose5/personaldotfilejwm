use std::process;
use nmrs::{NetworkManager, WifiSecurity};
use windui::prelude::*;

fn main() {
    // 1. Runtime de Tokio para las llamadas async a NetworkManager
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("No se pudo crear el runtime de Tokio");

    // 2. Conexión con NetworkManager
    let nm = rt
        .block_on(async { NetworkManager::new().await })
        .expect("NetworkManager no está disponible en el sistema");

    // 3. Obtener la lista de redes WiFi disponibles
    let networks = rt
        .block_on(async { nm.list_networks().await })
        .expect("Error al listar redes WiFi");

    // 4. Señales de estado para la UI
    let status = signal(String::from("Selecciona una red"));
    let selected_ssid = signal(None::<String>);
    let password = signal(String::new());

    // 5. App y canal para recibir mensajes desde el hilo de Tokio
    let mut app = App::new("WiFi", 200, 400);

    // El canal se crea antes de construir la UI (requiere &mut app)
    let tx = app.channel::<String>(move |_ctx, msg| {
        status.set(msg);
    });

    // 6. Construcción de la UI con Element (imperativo, retained-mode)
    let ui = Element::col()
        .fill()
        .padding(10)
        .spacing(8)
        // El contenedor raíz se cierra si pierde el foco (clic fuera de la ventana)
        .on_blur(|ctx| {
            ctx.force_close();
        })
        // Etiqueta de estado
        .child(Element::label_signal(status).font_size(14.0))
        // Lista de botones, uno por cada red WiFi encontrada
        .children(networks.into_iter().map(|net| {
            let ssid = net.ssid.clone();
            let strength = net.strength.unwrap_or(0);
            let ssid_clone = ssid.clone();
            Element::button(format!("{} ({}%)", ssid, strength)).on_click(move |_| {
                // Al hacer clic, se selecciona la red y se muestra el campo de contraseña
                selected_ssid.set(Some(ssid_clone.clone()));
            })
        }))
        // Campo de contraseña (solo visible cuando hay una red seleccionada)
        .child(
            Element::text_input(password, "Contraseña WiFi")
                .password()
                .visible_when(move || selected_ssid.get().is_some()),
        )
        // Botón para iniciar la conexión
        .child(
            Element::button("Conectar").on_click(move |_| {
                if let Some(ssid) = selected_ssid.get() {
                    let psk = password.get();
                    let nm_clone = nm.clone();
                    let rt_handle = rt.handle().clone();
                    let tx_clone = tx.clone();

                    // Actualizar estado inmediatamente en la UI
                    status.set(format!("Conectando a {}…", ssid));

                    // Lanzar la conexión en segundo plano
                    rt_handle.spawn(async move {
                        // Si la contraseña está vacía, se intenta como red abierta.
                        // Si no, se usa WPA-PSK.
                        let creds = if psk.is_empty() {
                            WifiSecurity::Open
                        } else {
                            WifiSecurity::WpaPsk { psk: psk.into() }
                        };

                        let resultado = nm_clone.connect(&ssid, creds).await;

                        // Enviar el resultado al hilo de la UI a través del canal
                        let mensaje = match resultado {
                            Ok(_) => format!("✅ Conectado a {}", ssid),
                            Err(e) => format!("❌ Error: {}", e),
                        };
                        let _ = tx_clone.send(mensaje);
                    });
                }
            }),
        );

    // 7. Asignar el contenido y ejecutar la aplicación
    app.content(ui).run();
}