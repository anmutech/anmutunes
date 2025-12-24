<script lang="ts">
  import iro from "@jaames/iro";
  import { config_state } from "../../state.svelte";
  import { onMount } from "svelte";
  import type { IroColorPicker } from "@jaames/iro/dist/ColorPicker";

  let { active }: { active: number } = $props();

  function get_color() {
    switch (active) {
      case 0:
        return config_state.custom_colors.background;
      case 1:
        return config_state.custom_colors.background_active;
      case 2:
        return config_state.custom_colors.background_hover;
      case 3:
        return config_state.custom_colors.background_button;
      case 4:
        return config_state.custom_colors.border_color;
      case 5:
        return config_state.custom_colors.accent_input;
      case 6:
        return config_state.custom_colors.warn;
      case 7:
        return config_state.custom_colors.text;
      case 8:
        return config_state.custom_colors.text_dim;
      case 9:
        return config_state.custom_colors.text_highlight;
      case 10:
        return config_state.custom_colors.shadow;

      default:
        return "#000000";
    }
  }

  function set_color(color: any) {
    console.log();
    switch (active) {
      case 0:
        config_state.custom_colors.background = color.hex8String;
        break;
      case 1:
        config_state.custom_colors.background_active = color.hex8String;
        break;
      case 2:
        config_state.custom_colors.background_hover = color.hex8String;
        break;
      case 3:
        config_state.custom_colors.background_button = color.hex8String;
        break;
      case 4:
        config_state.custom_colors.border_color = color.hex8String;
        break;
      case 5:
        config_state.custom_colors.accent_input = color.hex8String;
        break;
      case 6:
        config_state.custom_colors.warn = color.hex8String;
        break;
      case 7:
        config_state.custom_colors.text = color.hex8String;
        break;
      case 8:
        config_state.custom_colors.text_dim = color.hex8String;
        break;
      case 9:
        config_state.custom_colors.text_highlight = color.hex8String;
        break;
      case 10:
        config_state.custom_colors.shadow = color.hex8String;
        break;
      default:
        break;
    }
  }

  let colorPicker: IroColorPicker;
  onMount(() => {
    colorPicker = iro.ColorPicker("#picker", {
      color: get_color(),
      layout: [
        {
          component: iro.ui.Wheel,
        },
        { component: iro.ui.Slider },
        { component: iro.ui.Slider, options: { sliderType: "alpha" } },
      ],
    });
    colorPicker.on("color:change", set_color);
  });
</script>

<div class="content">
  <div id="picker"></div>
  <input
    type="text"
    value={get_color()}
    oninput={(e) => {
      let target = e.target as HTMLInputElement;
      // Regex from here:
      // https://stackoverflow.com/questions/1636350/how-to-identify-a-given-string-is-hex-color-format#comment130360899_1636354
      let hexRegex = /^#(?:[0-9a-fA-F]{2}){3,4}$/;
      if (hexRegex.test(target.value)) {
        colorPicker.color.set(target.value);
        set_color({ hex8String: target.value });
      }
    }}
  />
</div>

<style>
  .content {
    display: grid;
    grid-template-rows: auto auto;
    gap: 1rem;
  }
  input {
    width: 100%;
  }
</style>
