import * as stylex from '@stylexjs/stylex';
export const styles = stylex.create({
  root: {
    color: {
      default: 'black',
      '@media (min-width: 100px) and (max-width: 200px)': 'c0',
      '@media (min-width: 300px) and (max-width: 400px)': 'c1',
      '@media (min-width: 500px) and (max-width: 600px)': 'c2',
      '@media (min-width: 700px) and (max-width: 800px)': 'c3',
      '@media (min-width: 900px) and (max-width: 1000px)': 'c4',
      '@media (min-width: 1100px) and (max-width: 1200px)': 'c5',
      '@media (min-width: 1300px) and (max-width: 1400px)': 'c6',
      '@media (min-width: 1500px) and (max-width: 1600px)': 'c7',
      '@media (min-width: 1700px) and (max-width: 1800px)': 'c8',
      '@media (min-width: 1900px) and (max-width: 2000px)': 'c9',
      '@media (min-width: 2100px) and (max-width: 2200px)': 'c10',
      '@media (min-width: 2300px) and (max-width: 2400px)': 'c11',
      '@media (min-width: 2500px) and (max-width: 2600px)': 'c12',
    },
  },
});

