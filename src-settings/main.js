import Settings from "./Settings.svelte";
import { mount } from "svelte";

const app = mount(Settings, { target: document.getElementById("app") });

export default app;
