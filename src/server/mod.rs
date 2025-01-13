use std::sync::{Arc, Mutex};
use std::ops::DerefMut;
use std::time::Duration;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::thread;

#[derive(PartialEq)]
struct Alert {
    text: String,
    instrument: String,
    timeframe: String
}

#[derive(PartialEq)]
struct Alerts {
    key: String,
    strategy_name: String,
    server_name: String,
    notifications: Vec<Alert>
}

impl Alerts {
    fn add(&mut self, alert: Alert) {
        self.notifications.push(alert);
    }
}

struct HTTPJSONSender {
}

impl HTTPJSONSender {
    fn send(&self, url: &String, json: &String) {
        let client = Client::new();
        let resp = client.post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(json.clone())
            .send();
        match resp {
            Err(_e) => panic!(""),
            Ok(_r) => return
        }
    }
}

struct AlertSender {
    alerts: Arc<Mutex<Vec<Alerts>>>
}

impl AlertSender {
    pub fn new() -> Self {
        let mut new_instance = Self {
            alerts: Arc::new(Mutex::new(Vec::new()))
        };
        new_instance.do_work();
        return new_instance;
    }

    fn find_container<'a>(key: &String, alerts: &'a mut Vec<Alerts>) -> Option<&'a mut Alerts> {
        for alert_group in alerts.iter_mut() {
            if alert_group.key.eq(key) {
                return Some(alert_group);
            }
        }
        return None;
    }

    pub fn send_alert(&mut self, key: &String, text: &String, instrument: &String, timeframe: &String, url: &String) {
        let alert = Alert {
            text: String::from(text),
            instrument: String::from(instrument),
            timeframe: String::from(timeframe)
        };
        let mut alerts_data = self.alerts.lock().unwrap();
        let container = Self::find_container(key, alerts_data.deref_mut());
        match container {
            None => {
                let mut new_container = Alerts {
                    key: String::from(key),
                    server_name: String::from(url),
                    notifications: Vec::new(),
                    strategy_name: "".to_string()
                };
                new_container.add(alert);
                alerts_data.deref_mut().push(new_container);
            },
            Some(c) => {
                c.add(alert);
            }
        }
    }

    fn do_work(&mut self) {
        let alerts = Arc::clone(&self.alerts);
        let alert_sender = AlertSenderImpl::new();
		thread::spawn(move || {
            loop {
                {
                    let mut alerts_data = alerts.lock().unwrap();
                    for alert_group in alerts_data.iter() {
                        alert_sender.send_alerts(alert_group);
                    }
                    alerts_data.clear();
                }
                thread::sleep(Duration::from_secs(1));
            }
		});
    }
}

struct AlertSenderImpl {
    sender: HTTPJSONSender
}

impl AlertSenderImpl {
    fn new() -> Self {
        return Self {
            sender: HTTPJSONSender { }
        }
    }

    fn send_alerts(&self, alerts_container: &Alerts) {
        if alerts_container.notifications.len() == 0 {
            return;
        }
        if let Ok(json) = format_alerts(alerts_container) {
            let mut url = alerts_container.server_name.clone();
            const API_PATH: &str = "/api/v1/Notification";
            url.push_str(API_PATH);
            self.sender.send(&url, &json);
        }
    }
}

fn format_alerts(alerts: &Alerts) -> Result<String, bool> {
    let mut out = json::JsonValue::new_object();
    out["Key"] = alerts.key.clone().into();
	out["StrategyName"] = alerts.strategy_name.clone().into();
	out["Platform"] = "MetaTrader".into();

    let mut notifications = json::JsonValue::new_array();
    for notification in alerts.notifications.iter() {
        let mut alert = json::JsonValue::new_object();
		alert["Text"] = notification.text.clone().into();
		alert["Instrument"] = notification.instrument.clone().into();
		alert["TimeFrame"] = notification.timeframe.clone().into();
        if let Err(_r) = notifications.push(alert) {
            return Err(false);
        }
    }
    out["Notifications"] = notifications;
    return Ok(out.dump());
}

pub struct PRConnection {
    alerts_sender: AlertSender
}

impl PRConnection {
    pub fn new() -> Self {
        return Self {
            alerts_sender: AlertSender::new()
        };
    }
    pub fn send_alert(&mut self, key: &String, text: &String, instrument: &String, timeframe: &String, url: &String) {
        self.alerts_sender.send_alert(key, text, instrument, timeframe, url);
    }
}