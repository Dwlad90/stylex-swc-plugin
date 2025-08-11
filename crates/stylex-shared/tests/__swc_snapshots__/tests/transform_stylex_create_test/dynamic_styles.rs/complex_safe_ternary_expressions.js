import * as stylex from '@stylexjs/stylex';
export const styles = {
    root: (isDark, isLarge, isActive, width, height, color)=>[
            {
                kWkggS: "xr5ldyu",
                kMwMTN: "xfx01vb",
                kzqmXN: "x1bl4301",
                kZKoxP: "x1f5funs",
                kogj98: "x1cpkpif",
                kmVPX3: "x6rcfto",
                kGuDYH: "x6zurak",
                kSiTet: "xa0d40w",
                k3aq6I: "x1uosm7l",
                $$css: true
            },
            {
                "--backgroundColor": (isDark ? isLarge ? 'black' : 'gray' : isActive ? 'blue' : 'white') != null ? isDark ? isLarge ? 'black' : 'gray' : isActive ? 'blue' : 'white' : undefined,
                "--color": (isDark ? color || 'white' : color ?? 'black') != null ? isDark ? color || 'white' : color ?? 'black' : undefined,
                "--width": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isLarge ? width + 100 : width - 50),
                "--height": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isActive ? height * 2 : height / 2),
                "--margin": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isDark ? width + height || 20 : (width - height) ?? 10),
                "--padding": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isLarge ? width * height + 50 : width / height - 25),
                "--fontSize": ((val)=>typeof val === "number" ? val + "px" : val != null ? val : undefined)(isDark ? isLarge ? width + 20 : width - 10 : isActive ? height + 15 : height - 5),
                "--opacity": (isLarge ? isActive ? 1 : 0.8 : isDark ? 0.9 : 0.7) != null ? isLarge ? isActive ? 1 : 0.8 : isDark ? 0.9 : 0.7 : undefined,
                "--transform": (isActive ? isLarge ? 'scale(1.2)' : 'scale(1.1)' : isDark ? 'rotate(5deg)' : 'rotate(-5deg)') != null ? isActive ? isLarge ? 'scale(1.2)' : 'scale(1.1)' : isDark ? 'rotate(5deg)' : 'rotate(-5deg)' : undefined
            }
        ]
};
