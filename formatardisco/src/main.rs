extern crate native_windows_gui as nwg;
extern crate sysinfo;
use std::ptr::read_unaligned;
use std::ptr::read_unaligned;
use std::ptr::read_volatile;
use std::ptr::read;
use nwg::NativeUi;
use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use sysinfo::{System, SystemExt, DiskExt};

#[derive(Default)]
pub struct FormatarDiscoApp {
    window: nwg::Window,
    combo_box: nwg::ComboBox<String>,
    format_button: nwg::Button,
    layout: nwg::GridLayout,
}

impl fmt::Debug for FormatarDiscoApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormatarDiscoApp")
            .field("window", &"Window")
            .field("combo_box", &"ComboBox")
            .field("format_button", &"Button")
            .field("layout", &"GridLayout")
            .finish()
    }
}

impl FormatarDiscoApp {
    fn formatar_disco(&self) {
        if let Some(unidade) = self.combo_box.selection_string() {
            let message = format!("Tem certeza de que deseja formatar a unidade {}?", unidade);
            let params = nwg::MessageParams {
                title: "Confirmação",
                content: &message,
                buttons: nwg::MessageButtons::YesNo,
                icons: nwg::MessageIcons::Warning,
            };
            let confirmation = nwg::modal_message(&self.window.handle, &params);

            if confirmation == nwg::MessageChoice::Yes {
                let output = Command::new("cmd")
                    .args(&["/C", &format!("format {}: /Q /Y", unidade)])
                    .output()
                    .expect("Falha ao executar o comando de formatação");

                if output.status.success() {
                    nwg::simple_message("Sucesso", "Formatação concluída com sucesso!");
                } else {
                    nwg::simple_message("Erro", "Falha ao formatar o disco.");
                }
            }
        }
    }
}

impl nwg::NativeUi<Rc<RefCell<FormatarDiscoApp>>> for FormatarDiscoApp {
    fn build_ui(mut data: FormatarDiscoApp) -> Result<Rc<RefCell<FormatarDiscoApp>>, nwg::NwgError> {
        nwg::Window::builder()
            .size((300, 150))
            .position((300, 300))
            .title("Formatar Disco")
            .build(&mut data.window)?;

        let mut system = System::new_all();
        system.refresh_disks_list();
        let drives: Vec<String> = system.disks().iter().map(|disk| disk.mount_point().to_string_lossy().to_string()).collect();

        nwg::ComboBox::builder()
            .collection(drives)
            .parent(&data.window)
            .build(&mut data.combo_box)?;

        nwg::Button::builder()
            .text("Formatar")
            .parent(&data.window)
            .build(&mut data.format_button)?;

        nwg::GridLayout::builder()
            .parent(&data.window)
            .spacing(1)
            .child(0, 0, &data.combo_box)
            .child(1, 0, &data.format_button)
            .build(&mut data.layout)?;

        let data = Rc::new(RefCell::new(data));
        let data_ref = Rc::clone(&data);
        let format_button_handle = data.borrow().format_button.handle;

        nwg::bind_event_handler(&data.borrow().window.handle, &data.borrow().window.handle, move |evt, _evt_data, handle| {
            if evt == nwg::Event::OnButtonClick && handle == format_button_handle {
                data_ref.borrow().formatar_disco();
            }
        });

        Ok(data)
    }
}

fn main() {
    // Supondo que USER_SHARED_DATA seja um ponteiro para uma estrutura packed
    let tick_count_quad = unsafe { read_unaligned(&(*USER_SHARED_DATA).u.TickCountQuad as *const _ as *const u64) };
    println!("TickCountQuad: {}", tick_count_quad);
    

    // Código existente
    nwg::init().expect("Falha ao inicializar a biblioteca Native Windows GUI");
    let _app = FormatarDiscoApp::build_ui(Default::default()).expect("Falha ao construir a interface do usuário");
    nwg::dispatch_thread_events();
}