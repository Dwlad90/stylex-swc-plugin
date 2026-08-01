/**
 * @jest-environment jsdom
 */
import { describe, expect, it } from '@jest/globals';
import { render } from '@testing-library/react';
import Home from '@/app/page';

describe('Home', () => {
  it('render styles successfully', () => {
    const { container } = render(<Home />);

    expect(container).toMatchSnapshot();
  });
});
