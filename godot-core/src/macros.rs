/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![macro_use]

#[doc(hidden)]
#[macro_export]
macro_rules! gdext_call_signature_method {
    (
        ptrcall,
        $Signature:ty,
        $instance_ptr:ident, $args:ident, $ret:ident,
        $func:expr,
        $method_name:ident,
        $ptrcall_type:path
    ) => {
        <$Signature as $crate::builtin::meta::PtrcallSignatureTuple>::ptrcall(
            $instance_ptr,
            $args,
            $ret,
            $func,
            stringify!($method_name),
            $ptrcall_type,
        );
    };

    (
        varcall,
        $Signature:ty,
        $instance_ptr:ident, $args:ident, $ret:ident, $err:ident,
        $func:expr,
        $method_name:ident
    ) => {
        <$Signature as $crate::builtin::meta::VarcallSignatureTuple>::varcall(
            $instance_ptr,
            $args,
            $ret,
            $err,
            $func,
            stringify!($method_name),
        );
    };
}
