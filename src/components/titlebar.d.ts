import { RouteLocationRaw } from "vue-router";

export interface TitlebarCommandEvent {
  original_event: Event;
  item: TitlebarItem;
}

export interface TitlebarItem {
  label?: string;
  separator?: boolean;
  items?: TitlebarItem[];
  icon?: string;
  route?: RouteLocationRaw;
  command?: (event: TitlebarCommandEvent) => void;
  disabled?: boolean;
  severity?: "secondary" | "info" | "success" | "warn" | "danger" | "contrast";
  plain?: boolean;
}

export interface TitlebarDef {
  start?: TitlebarItem[];
  end?: TitlebarItem[];
}
