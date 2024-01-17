use tfhe::integer::gen_keys;
use tfhe::integer::wopbs::*;
use tfhe::shortint::parameters::parameters_wopbs_message_carry::WOPBS_PARAM_MESSAGE_4_CARRY_4_KS_PBS;
use tfhe::shortint::parameters::PARAM_MESSAGE_4_CARRY_4_KS_PBS;

fn main() {
    let basis: Vec<u64> = vec![13, 14, 15];

    let (cks, sks) = gen_keys(PARAM_MESSAGE_4_CARRY_4_KS_PBS);
    let wopbs_key = WopbsKey::new_wopbs_key(&cks, &sks, &WOPBS_PARAM_MESSAGE_4_CARRY_4_KS_PBS);

    let mut msg_space = 1;
    for modulus in basis.iter() {
        msg_space *= modulus;
    }
    let clear = 42 % msg_space;
    let threshold = 30;
    let ct = cks.encrypt_crt(clear, basis.clone());
    let ct = wopbs_key.keyswitch_to_wopbs_params(&sks, &ct);
    let lut = wopbs_key.generate_lut_crt(&ct, |x| if x > threshold {1} else {0});
    let ct_res = wopbs_key.wopbs(&ct, &lut);
    let ct_res = wopbs_key.keyswitch_to_pbs_params(&ct_res);
    let res = cks.decrypt_crt(&ct_res);
    assert_eq!(res, if clear > threshold {1} else {0});
}
