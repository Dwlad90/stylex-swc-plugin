import { describe, it, expect } from 'vitest';
import { composeStories } from '@storybook/react';
import { render } from '@testing-library/react';
import * as ButtonStories from './Button.stories';

describe('Button Snapshots', () => {
  const stories = composeStories(ButtonStories);

  Object.entries(stories).forEach(([name, Story]) => {
    it(`renders ${name} story correctly`, () => {
      const { container } = render(<Story />);
      expect(container.firstChild).toMatchSnapshot();
    });
  });
});
