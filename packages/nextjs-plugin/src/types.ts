export type PluginRule = {
  class_name: string;
  style: { ltr: string; rtl?: null | string };
  priority: number;
};
