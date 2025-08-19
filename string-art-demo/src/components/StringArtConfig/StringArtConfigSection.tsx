import { useEffect, useState, type RefObject } from "react";
import { type StringArtConfig } from "../../hooks/useStringArt";

const Slider = ({
  title,
  settingsRef,
  index,
}: {
  title: string;
  settingsRef: RefObject<StringArtConfig>;
  index: keyof StringArtConfig;
}) => {
  const minMaxVals: Record<
    keyof Omit<
      StringArtConfig,
      | "extract_subject"
      | "remove_shadows"
      | "preserve_eyes"
      | "preserve_negative_space"
    >,
    [number, number, number, number]
  > = {
    //min, max, start
    image_size: [500, 2000, 500, 100],
    line_darkness: [25, 300, 100, 5],
    max_lines: [800, 5000, 1000, 100],
    min_improvement_score: [0, 100, 15, 1],
    negative_space_penalty: [0, 100, 0, 1],
    negative_space_threshold: [0, 100, 0, 1],
    num_nails: [360, 1440, 360, 360/4],
    progress_frequency: [200, 500, 200, 50],
  };

  const [val, setVal] = useState("0");
  const dontRender = !index || !Object.keys(minMaxVals).includes(index as string)
  const [min, max, start, step] = dontRender? [0,0,0,0] : minMaxVals[index as keyof typeof minMaxVals];

  useEffect(() => {
    setVal(String(start))
  }, [start])

  if (dontRender) return null;
  if (!settingsRef?.current) return null;
  return (
    <div key={index}>
      <input
        defaultValue={start}
        type="range"
        id="volume"
        name="volume"
        min={min}
        max={max}
        step={step}
        onChange={({ target }) => {
          (settingsRef.current[index] as unknown) = target.value;
          setVal(target.value);
        }}
      />
      <label htmlFor="volume">{title} ({val})</label>
    </div>
  );
};

const StringArtConfigSection = ({
  settings,
}: {
  settings: RefObject<StringArtConfig>;
}) => {
  if (!settings.current) {
    return null;
  }
  return (
    <section>
      <h2>String Art Configuration</h2>
      {(Object.keys(settings.current) as (keyof StringArtConfig)[]).map(
        (key) => {
          return (
            <Slider
              key={key}
              index={key}
              title={key.split("_").join(" ")}
              settingsRef={settings}
            />
          );
        }
      )}
    </section>
  );
};

export default StringArtConfigSection;
