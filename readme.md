# Advanced notifications library for MetaTrader 4/5

Support sending alerts into Telegram/Discord asynchronosly. 

## Usage

'''
extern string   advanced_key             = ""; // Advanced alert key
//somewhere in the file, like after input block
#import "AdvancedNotificationsLib.dll"
void AdvancedAlert(string key, string text, string instrument, string timeframe);
void AdvancedAlertCustom(string key, string text, string instrument, string timeframe, string url);
#import

void OnTick()
{
   if (NeedToSendAlert())//your condition for sending an alert
   {
      AdvancedAlert(advanced_key, "Do something", _Symbol, TimeFrameToString(_Period));
      //or
      AdvancedAlertCustom(advanced_key, "Do something", _Symbol, TimeFrameToString(_Period), "https://profitrobots.com");
   }
}

'''

## Pro and cons

Alternatevely you can send the telegram alert like this:

'''

   string cookie = NULL, headers;
   char post[], result[];
   string base_url = "https://api.telegram.org";
   string url = "https://api.telegram.org/bot" + token + "/sendMessage?chat_id=" + chat_id + "&text=" + text;
   ResetLastError();
   int timeout = 2000;
   int res = WebRequest("GET", url, cookie, NULL, timeout, post, 0, result, headers);
   if(res == -1)
   {
      //handle error
   }
'''

But this method can freeze you Metatrader up to choosen timeout (2 seconds in this case). And if the call will fail (which can happen from time to time) you will lose that alert. You can add retry logic into you code, but this can freeze your Metatrader for even longer.

This library, on other hand, do it asynchronosly. It puts your alert in a queue and sends it in a separate thread. And if the call will fail it will try to resend the alert for you. 

## Support

You can support us on [boosty](https://boosty.to/tsconvertor).