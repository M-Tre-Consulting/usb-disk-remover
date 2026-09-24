use crate::I18nStrings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    En,
    It,
    De,
    Fr,
    Ar,
    Ru,
}

impl Language {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Language::En,
            1 => Language::It,
            2 => Language::De,
            3 => Language::Fr,
            4 => Language::Ar,
            5 => Language::Ru,
            _ => Language::En,
        }
    }

    pub fn to_index(self) -> usize {
        match self {
            Language::En => 0,
            Language::It => 1,
            Language::De => 2,
            Language::Fr => 3,
            Language::Ar => 4,
            Language::Ru => 5,
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "it" => Language::It,
            "de" => Language::De,
            "fr" => Language::Fr,
            "ar" => Language::Ar,
            "ru" => Language::Ru,
            _ => Language::En,
        }
    }

    pub fn to_code(self) -> &'static str {
        match self {
            Language::En => "en",
            Language::It => "it",
            Language::De => "de",
            Language::Fr => "fr",
            Language::Ar => "ar",
            Language::Ru => "ru",
        }
    }

    pub fn get_strings(self) -> I18nStrings {
        match self {
            Language::En => I18nStrings {
                settings_btn: "Settings".into(),
                about_btn: "About".into(),
                refresh_btn: "Refresh".into(),
                scanning: "Scanning drives...".into(),
                no_drives_title: "No removable drives detected".into(),
                no_drives_desc: "Connect a USB flash drive or external drive to manage it.".into(),
                scan_for_drives: "Scan for drives".into(),
                unlabeled_volume: "Unlabeled volume".into(),
                storage_device: "Storage device".into(),
                safely_remove: "Safely Remove".into(),
                removing: "Removing...".into(),
                ready: "Ready".into(),
                settings_title: "Settings".into(),
                settings_desc: "Configure startup, language, and window behavior".into(),
                language_label: "Language".into(),
                autostart_title: "Start automatically with Windows".into(),
                autostart_desc: "Launch application on system startup (Run Registry key)".into(),
                start_minimized_title: "Start minimized to system tray".into(),
                start_minimized_desc: "Do not show main window on startup, stay in notification area".into(),
                close_to_tray_title: "Minimize to tray on close (✕)".into(),
                close_to_tray_desc: "Closing the window hides the application instead of quitting".into(),
                save_btn: "Save".into(),
                close_btn: "Close".into(),
                about_desc: "A lightweight Windows utility for safely ejecting removable USB and Firewire drives.".into(),
                about_ref: "A Rust port of USB Disk Ejector by QuickAndEasySoftware.".into(),
                tray_open: "Open USB Disk Remover".into(),
                tray_settings: "Settings...".into(),
                tray_about: "About...".into(),
                tray_quit: "Quit".into(),
            },
            Language::It => I18nStrings {
                settings_btn: "Impostazioni".into(),
                about_btn: "Info".into(),
                refresh_btn: "Aggiorna".into(),
                scanning: "Scansione unità in corso...".into(),
                no_drives_title: "Nessuna unità rimovibile rilevata".into(),
                no_drives_desc: "Collega una chiavetta USB o un disco esterno per gestirlo.".into(),
                scan_for_drives: "Cerca unità".into(),
                unlabeled_volume: "Unità senza etichetta".into(),
                storage_device: "Dispositivo di archiviazione".into(),
                safely_remove: "Rimuovi in sicurezza".into(),
                removing: "Rimozione in corso...".into(),
                ready: "Pronto".into(),
                settings_title: "Impostazioni".into(),
                settings_desc: "Configura avvio, lingua e comportamento della finestra".into(),
                language_label: "Lingua".into(),
                autostart_title: "Avvia automaticamente con Windows".into(),
                autostart_desc: "Esegue il programma all'avvio del sistema (Registro Run)".into(),
                start_minimized_title: "Avvia ridotto a icona nella tray".into(),
                start_minimized_desc: "All'avvio non mostra la finestra, resta nell'area di notifica".into(),
                close_to_tray_title: "Riduci nella tray alla chiusura (✕)".into(),
                close_to_tray_desc: "Il pulsante ✕ nasconde l'applicazione invece di chiuderla".into(),
                save_btn: "Salva".into(),
                close_btn: "Chiudi".into(),
                about_desc: "Utility nativa per Windows per espellere in sicurezza le unità USB e Firewire.".into(),
                about_ref: "Porting in Rust di USB Disk Ejector di QuickAndEasySoftware.".into(),
                tray_open: "Apri USB Disk Remover".into(),
                tray_settings: "Impostazioni...".into(),
                tray_about: "Informazioni...".into(),
                tray_quit: "Esci".into(),
            },
            Language::De => I18nStrings {
                settings_btn: "Einstellungen".into(),
                about_btn: "Über".into(),
                refresh_btn: "Aktualisieren".into(),
                scanning: "Laufwerke werden gescannt...".into(),
                no_drives_title: "Keine Wechsellaufwerke erkannt".into(),
                no_drives_desc: "Schließen Sie ein USB- oder Firewire-Laufwerk an, um es zu verwalten.".into(),
                scan_for_drives: "Nach Laufwerken suchen".into(),
                unlabeled_volume: "Unbenanntes Volume".into(),
                storage_device: "Speichergerät".into(),
                safely_remove: "Sicher entfernen".into(),
                removing: "Wird entfernt...".into(),
                ready: "Bereit".into(),
                settings_title: "Einstellungen".into(),
                settings_desc: "Start, Sprache und Fensterverhalten konfigurieren".into(),
                language_label: "Sprache".into(),
                autostart_title: "Automatisch mit Windows starten".into(),
                autostart_desc: "Anwendung beim Systemstart ausführen (Registry-Eintrag)".into(),
                start_minimized_title: "Minimiert im Infobereich starten".into(),
                start_minimized_desc: "Hauptfenster beim Start nicht anzeigen, im Infobereich bleiben".into(),
                close_to_tray_title: "Beim Schließen minimieren (✕)".into(),
                close_to_tray_desc: "Schließen blendet die Anwendung im Infobereich aus".into(),
                save_btn: "Speichern".into(),
                close_btn: "Schließen".into(),
                about_desc: "Ein schlankes Windows-Tool zum sicheren Auswerfen von USB- und Firewire-Laufwerken.".into(),
                about_ref: "Ein Rust-Port von USB Disk Ejector von QuickAndEasySoftware.".into(),
                tray_open: "USB Disk Remover öffnen".into(),
                tray_settings: "Einstellungen...".into(),
                tray_about: "Über...".into(),
                tray_quit: "Beenden".into(),
            },
            Language::Fr => I18nStrings {
                settings_btn: "Paramètres".into(),
                about_btn: "À propos".into(),
                refresh_btn: "Actualiser".into(),
                scanning: "Analyse des lecteurs...".into(),
                no_drives_title: "Aucun lecteur amovible détecté".into(),
                no_drives_desc: "Connectez une clé USB ou un disque externe pour le gérer.".into(),
                scan_for_drives: "Rechercher des lecteurs".into(),
                unlabeled_volume: "Volume sans nom".into(),
                storage_device: "Périphérique de stockage".into(),
                safely_remove: "Retirer en toute sécurité".into(),
                removing: "Suppression en cours...".into(),
                ready: "Prêt".into(),
                settings_title: "Paramètres".into(),
                settings_desc: "Configurer le démarrage, la langue et la fenêtre".into(),
                language_label: "Langue".into(),
                autostart_title: "Démarrer automatiquement avec Windows".into(),
                autostart_desc: "Lancer l'application au démarrage du système (Registre)".into(),
                start_minimized_title: "Démarrer minimisé dans la barre".into(),
                start_minimized_desc: "Ne pas afficher la fenêtre au démarrage, rester dans la barre".into(),
                close_to_tray_title: "Réduire dans la barre à la fermeture (✕)".into(),
                close_to_tray_desc: "Fermer la fenêtre masque l'application au lieu de la quitter".into(),
                save_btn: "Enregistrer".into(),
                close_btn: "Fermer".into(),
                about_desc: "Un utilitaire Windows léger pour éjecter les lecteurs USB et Firewire en sécurité.".into(),
                about_ref: "Un portage en Rust de USB Disk Ejector par QuickAndEasySoftware.".into(),
                tray_open: "Ouvrir USB Disk Remover".into(),
                tray_settings: "Paramètres...".into(),
                tray_about: "À propos...".into(),
                tray_quit: "Quitter".into(),
            },
            Language::Ar => I18nStrings {
                settings_btn: "الإعدادات".into(),
                about_btn: "حول".into(),
                refresh_btn: "تحديث".into(),
                scanning: "جارٍ فحص محركات الأقراص...".into(),
                no_drives_title: "لم يتم اكتشاف محركات أقراص قابلة للإزالة".into(),
                no_drives_desc: "قم بتوصيل محرك أقراص USB أو محرك أقراص خارجي لإدارته.".into(),
                scan_for_drives: "البحث عن محركات الأقراص".into(),
                unlabeled_volume: "وحدة تخزين بدون تسمية".into(),
                storage_device: "جهاز تخزين".into(),
                safely_remove: "إزالة بأمان".into(),
                removing: "جارٍ الإزالة...".into(),
                ready: "جاهز".into(),
                settings_title: "الإعدادات".into(),
                settings_desc: "تكوين بدء التشغيل واللغة وسلوك النافذة".into(),
                language_label: "اللغة".into(),
                autostart_title: "بدء التشغيل تلقائيًا مع Windows".into(),
                autostart_desc: "تشغيل التطبيق تلقائيًا عند بدء النظام (مفتاح التسجيل)".into(),
                start_minimized_title: "بدء التشغيل مصغرًا في علبة النظام".into(),
                start_minimized_desc: "عدم إظهار النافذة عند بدء التشغيل، البقاء في علبة النظام".into(),
                close_to_tray_title: "التصغير إلى علبة النظام عند الإغلاق (✕)".into(),
                close_to_tray_desc: "إغلاق النافذة يخفي التطبيق في علبة النظام بدلاً من إنهائه".into(),
                save_btn: "حفظ".into(),
                close_btn: "إغلاق".into(),
                about_desc: "أداة خفيفة لنظام Windows لإزالة محركات أقراص USB وFirewire بأمان.".into(),
                about_ref: "نسخة Rust من برنامج USB Disk Ejector لشركة QuickAndEasySoftware.".into(),
                tray_open: "فتح USB Disk Remover".into(),
                tray_settings: "الإعدادات...".into(),
                tray_about: "حول...".into(),
                tray_quit: "خروج".into(),
            },
            Language::Ru => I18nStrings {
                settings_btn: "Настройки".into(),
                about_btn: "О программе".into(),
                refresh_btn: "Обновить".into(),
                scanning: "Сканирование дисков...".into(),
                no_drives_title: "Съемные диски не обнаружены".into(),
                no_drives_desc: "Подключите USB-накопитель или внешний диск для управления.".into(),
                scan_for_drives: "Поиск дисков".into(),
                unlabeled_volume: "Том без метки".into(),
                storage_device: "Устройство хранения".into(),
                safely_remove: "Безопасное извлечение".into(),
                removing: "Извлечение...".into(),
                ready: "Готово".into(),
                settings_title: "Настройки".into(),
                settings_desc: "Настройка автозапуска, языка и поведения окна".into(),
                language_label: "Язык".into(),
                autostart_title: "Запуск вместе с Windows".into(),
                autostart_desc: "Запуск приложения при старте системы (ветка реестра Run)".into(),
                start_minimized_title: "Запуск свернутым в трей".into(),
                start_minimized_desc: "Не показывать окно при запуске, оставаться в области уведомлений".into(),
                close_to_tray_title: "Сворачивать в трей при закрытии (✕)".into(),
                close_to_tray_desc: "Нажатие на ✕ скрывает окно в трей вместо завершения программы".into(),
                save_btn: "Сохранить".into(),
                close_btn: "Закрыть".into(),
                about_desc: "Легкая утилита для безопасного извлечения USB и Firewire накопителей в Windows.".into(),
                about_ref: "Порт на Rust утилиты USB Disk Ejector от QuickAndEasySoftware.".into(),
                tray_open: "Открыть USB Disk Remover".into(),
                tray_settings: "Настройки...".into(),
                tray_about: "О программе...".into(),
                tray_quit: "Выход".into(),
            },
        }
    }

    pub fn drives_detected(self, count: usize) -> String {
        match self {
            Language::En => {
                if count == 1 {
                    "1 drive detected.".to_string()
                } else {
                    format!("{} drives detected.", count)
                }
            }
            Language::It => {
                if count == 1 {
                    "1 unità rilevata.".to_string()
                } else {
                    format!("{} unità rilevate.", count)
                }
            }
            Language::De => {
                if count == 1 {
                    "1 Laufwerk erkannt.".to_string()
                } else {
                    format!("{} Laufwerke erkannt.", count)
                }
            }
            Language::Fr => {
                if count == 1 {
                    "1 lecteur détecté.".to_string()
                } else {
                    format!("{} lecteurs détectés.", count)
                }
            }
            Language::Ar => {
                if count == 1 {
                    "تم اكتشاف محرك أقراص واحد.".to_string()
                } else {
                    format!("تم اكتشاف {} محركات أقراص.", count)
                }
            }
            Language::Ru => {
                if count == 1 {
                    "Обнаружен 1 диск.".to_string()
                } else {
                    format!("Обнаружено {} дисков.", count)
                }
            }
        }
    }

    pub fn safely_removing(self, mount_point: &str) -> String {
        match self {
            Language::En => format!("Safely removing {}...", mount_point),
            Language::It => format!("Rimozione sicura di {} in corso...", mount_point),
            Language::De => format!("{} wird sicher entfernt...", mount_point),
            Language::Fr => format!("Retrait de {} en toute sécurité...", mount_point),
            Language::Ar => format!("جارٍ إزالة {} بأمان...", mount_point),
            Language::Ru => format!("Безопасное извлечение {}...", mount_point),
        }
    }

    pub fn safely_removed(self, mount_point: &str) -> String {
        match self {
            Language::En => format!("{} safely removed.", mount_point),
            Language::It => format!("{} rimossa con successo.", mount_point),
            Language::De => format!("{} sicher entfernt.", mount_point),
            Language::Fr => format!("{} retiré en toute sécurité.", mount_point),
            Language::Ar => format!("تمت إزالة {} بأمان.", mount_point),
            Language::Ru => format!("{} успешно извлечен.", mount_point),
        }
    }

    pub fn eject_error(self, mount_point: &str, err: &str) -> String {
        match self {
            Language::En => format!("Error ejecting {}: {}", mount_point, err),
            Language::It => format!("Errore espulsione {}: {}", mount_point, err),
            Language::De => format!("Fehler beim Auswerfen von {}: {}", mount_point, err),
            Language::Fr => format!("Erreur lors de l'éjection de {}: {}", mount_point, err),
            Language::Ar => format!("خطأ أثناء إخراج {}: {}", mount_point, err),
            Language::Ru => format!("Ошибка извлечения {}: {}", mount_point, err),
        }
    }

    pub fn drive_not_found(self, mount_point: &str) -> String {
        match self {
            Language::En => format!("Drive {} not found.", mount_point),
            Language::It => format!("Unità {} non trovata.", mount_point),
            Language::De => format!("Laufwerk {} nicht gefunden.", mount_point),
            Language::Fr => format!("Lecteur {} introuvable.", mount_point),
            Language::Ar => format!("محرك الأقراص {} غير موجود.", mount_point),
            Language::Ru => format!("Диск {} не найден.", mount_point),
        }
    }
}
