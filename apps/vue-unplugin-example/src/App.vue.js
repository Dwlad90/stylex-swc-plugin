/// <reference types="../node_modules/.vue-global-types/vue_3.5_false.d.ts" />
import { colors } from '@stylexjs/open-props/lib/colors.stylex';
import { fonts } from '@stylexjs/open-props/lib/fonts.stylex';
import { sizes } from '@stylexjs/open-props/lib/sizes.stylex';
import * as stylex from '@stylexjs/stylex';
const styles = stylex.create({
    container: {
        alignItems: 'center',
        backgroundColor: colors.choco3,
        display: 'grid',
        height: '100dvh',
        justifyContent: 'center',
        width: '100dvw',
    },
    button: {
        alignItems: 'center',
        backgroundColor: colors.jungle6,
        borderRadius: sizes.spacing15,
        color: colors.gray2,
        display: 'flex',
        fontFamily: fonts.mono,
        justifyContent: 'center',
        paddingBlock: sizes.spacing5,
        paddingInline: sizes.spacing10,
        borderWidth: 0,
        fontWeight: 'bold',
        ":hover": {
            backgroundColor: colors.jungle8,
        },
        ":active": {
            backgroundColor: colors.jungle10,
        }
    },
});
; /* PartiallyEnd: #3632/both.vue */
export default await (async () => {
    const { defineProps, defineSlots, defineEmits, defineExpose, defineModel, defineOptions, withDefaults, } = await import('vue');
    ; /* PartiallyEnd: #3632/scriptSetup.vue */
    const __VLS_fnComponent = (await import('vue')).defineComponent({});
    ;
    let __VLS_functionalComponentProps;
    function __VLS_template() {
        const __VLS_ctx = {};
        const __VLS_localComponents = {
            ...{},
            ...{},
            ...__VLS_ctx,
        };
        let __VLS_components;
        const __VLS_localDirectives = {
            ...{},
            ...__VLS_ctx,
        };
        let __VLS_directives;
        let __VLS_styleScopedClasses;
        let __VLS_resolvedLocalAndGlobalComponents;
        __VLS_elementAsFunction(__VLS_intrinsicElements.main, __VLS_intrinsicElements.main)({ ...{ class: ((__VLS_ctx.stylex.props(__VLS_ctx.styles.container).className)) }, });
        __VLS_elementAsFunction(__VLS_intrinsicElements.button, __VLS_intrinsicElements.button)({ ...{ class: ((__VLS_ctx.stylex.props(__VLS_ctx.styles.button).className)) }, });
        var __VLS_slots;
        var __VLS_inheritedAttrs;
        const __VLS_refs = {};
        var $refs;
        var $el;
        return {
            attrs: {},
            slots: __VLS_slots,
            refs: $refs,
            rootEl: $el,
        };
    }
    ;
    const __VLS_self = (await import('vue')).defineComponent({
        setup() {
            return {
                stylex: stylex,
                styles: styles,
            };
        },
    });
    return (await import('vue')).defineComponent({
        setup() {
            return {};
        },
        __typeEl: {},
    });
})(); /* PartiallyEnd: #3632/script.vue */
; /* PartiallyEnd: #4569/main.vue */
