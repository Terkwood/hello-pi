import { expect } from 'chai';
import { h } from 'preact';
import { mount } from 'enzyme';

import Header from '../src/components/header';

describe('Header', () => {
    it('should display something', () => {
        const wrapper = mount(h(Header, {}));
        expect(wrapper.text()).to.include('🍞');
    });
});