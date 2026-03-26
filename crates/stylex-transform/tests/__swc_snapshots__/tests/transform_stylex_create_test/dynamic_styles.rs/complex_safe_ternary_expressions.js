import * as stylex from '@stylexjs/stylex';
const _temp = {
    kWkggS: "xl8spv7",
    kMwMTN: "x14rh7hd",
    kzqmXN: "x5lhr3w",
    kZKoxP: "x16ye13r",
    kogj98: "xb9ncqk",
    kmVPX3: "x1fozly0",
    kGuDYH: "xdmh292",
    kSiTet: "xb4nw82",
    k3aq6I: "xsqj5wx",
    $$css: true
};
export const styles = {
    root: (isDark, isLarge, isActive, width, height, color)=>[
            _temp,
            {
                "--x-backgroundColor": (isDark ? isLarge ? 'black' : 'gray' : isActive ? 'blue' : 'white') != null ? isDark ? isLarge ? 'black' : 'gray' : isActive ? 'blue' : 'white' : undefined,
                "--x-color": (isDark ? color || 'white' : color ?? 'black') != null ? isDark ? color || 'white' : color ?? 'black' : undefined,
                "--x-width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isLarge ? width + 100 : width - 50),
                "--x-height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isActive ? height * 2 : height / 2),
                "--x-margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isDark ? width + height || 20 : (width - height) ?? 10),
                "--x-padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isLarge ? width * height + 50 : width / height - 25),
                "--x-fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isDark ? isLarge ? width + 20 : width - 10 : isActive ? height + 15 : height - 5),
                "--x-opacity": (isLarge ? isActive ? 1 : 0.8 : isDark ? 0.9 : 0.7) != null ? isLarge ? isActive ? 1 : 0.8 : isDark ? 0.9 : 0.7 : undefined,
                "--x-transform": (isActive ? isLarge ? 'scale(1.2)' : 'scale(1.1)' : isDark ? 'rotate(5deg)' : 'rotate(-5deg)') != null ? isActive ? isLarge ? 'scale(1.2)' : 'scale(1.1)' : isDark ? 'rotate(5deg)' : 'rotate(-5deg)' : undefined
            }
        ]
};
