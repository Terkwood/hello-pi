import { expect } from 'chai';
import { mount } from 'enzyme';
import Songs from '../src/routes/songs';
describe('Songs', () => {
    it('should display time', () => {
        const wrapper = mount(h(Songs, null));
        expect(wrapper.text()).to.include('Current time:');
    });
});
//# sourceMappingURL=Songs-test.js.map