import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import { hub } from "dom-native";
import type { HubEvent } from "./bindings/index";

listen("HubEvent", function (evt: TauriEvent<HubEvent<any>>) {
  const hubEvent = evt.payload;

  let _hub = hub(hubEvent.hub);

  if (hubEvent.label != null) {
    _hub.pub(hubEvent.topic, hubEvent.label, hubEvent.data);
  } else {
    _hub.pub(hubEvent.topic, hubEvent.data);
  }
});
