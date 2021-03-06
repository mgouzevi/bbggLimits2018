CMS_hhbbgg_13TeV_mgg_bkg_par1_cat2[0.9, -100, 100];

mgg_sig_m0_cat2[125., 122, 127];
mgg_sig_sigma_cat2[2.8, 2.1, 5.0];
mgg_sig_alpha1_cat2[1.5, 1.00, 10.0];
mgg_sig_n1_cat2[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat2[1.5, 1.00, 10.0];
mgg_sig_n2_cat2[2.0, 0.1, 10.0];
mggSig_cat2 = RooDoubleCB(mgg, mgg_sig_m0_cat2, mgg_sig_sigma_cat2, mgg_sig_alpha1_cat2, mgg_sig_n1_cat2, mgg_sig_alpha2_cat2, mgg_sig_n2_cat2);

mgg_hig_m0_ggh_cat2[124.2, 123, 125];
mgg_hig_sigma_ggh_cat2[2.8, 2.1, 5.0];
mgg_hig_alpha1_ggh_cat2[1.5, 1.00, 10.0];
mgg_hig_n1_ggh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat2[1.5, 1.00, 10.0];
mgg_hig_n2_ggh_cat2[2.0, 0.1, 10.0];
mggHig_ggh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat2, mgg_hig_sigma_ggh_cat2, mgg_hig_alpha1_ggh_cat2, mgg_hig_n1_ggh_cat2, mgg_hig_alpha2_ggh_cat2, mgg_hig_n2_ggh_cat2);

mgg_hig_m0_tth_cat2[124.2, 123, 125];
mgg_hig_sigma_tth_cat2[2.8, 2.1, 5.0];
mgg_hig_alpha1_tth_cat2[1.5, 1.00, 10.0];
mgg_hig_n1_tth_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat2[1.5, 1.00, 10.0];
mgg_hig_n2_tth_cat2[2.0, 0.1, 10.0];
mggHig_tth_cat2 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat2, mgg_hig_sigma_tth_cat2, mgg_hig_alpha1_tth_cat2, mgg_hig_n1_tth_cat2, mgg_hig_alpha2_tth_cat2, mgg_hig_n2_tth_cat2);

mgg_hig_m0_vh_cat2[124.2, 123, 125];
mgg_hig_sigma_vh_cat2[2.8, 2.1, 5.0];
mgg_hig_alpha1_vh_cat2[1.5, 1.00, 10.0];
mgg_hig_n1_vh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat2[1.5, 1.00, 10.0];
mgg_hig_n2_vh_cat2[2.0, 0.1, 10.0];
mggHig_vh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat2, mgg_hig_sigma_vh_cat2, mgg_hig_alpha1_vh_cat2, mgg_hig_n1_vh_cat2, mgg_hig_alpha2_vh_cat2, mgg_hig_n2_vh_cat2);

mgg_hig_m0_vbf_cat2[124.2, 123, 125];
mgg_hig_sigma_vbf_cat2[2.8, 2.1, 5.0];
mgg_hig_alpha1_vbf_cat2[1.5, 1.00, 10.0];
mgg_hig_n1_vbf_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat2[1.5, 1.00, 10.0];
mgg_hig_n2_vbf_cat2[2.0, 0.1, 10.0];
mggHig_vbf_cat2 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat2, mgg_hig_sigma_vbf_cat2, mgg_hig_alpha1_vbf_cat2, mgg_hig_n1_vbf_cat2, mgg_hig_alpha2_vbf_cat2, mgg_hig_n2_vbf_cat2);

mgg_hig_m0_bbh_cat2[124.2, 123, 125];
mgg_hig_sigma_bbh_cat2[2.8, 2.1, 5.0];
mgg_hig_alpha1_bbh_cat2[1.5, 1.00, 10.0];
mgg_hig_n1_bbh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat2[1.5, 1.00, 10.0];
mgg_hig_n2_bbh_cat2[2.0, 0.1, 10.0];
mggHig_bbh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat2, mgg_hig_sigma_bbh_cat2, mgg_hig_alpha1_bbh_cat2, mgg_hig_n1_bbh_cat2, mgg_hig_alpha2_bbh_cat2, mgg_hig_n2_bbh_cat2);


mjj_sig_m0_cat2[122.0, 115, 130];
mjj_sig_sigma_cat2[20.0, 10.0, 60.0];
mjj_sig_alpha1_cat2[0.3, 0.2, 2.0];
mjj_sig_n1_cat2[1.0, 0.5, 10.0];
mjj_sig_alpha2_cat2[3, 0.5, 2.0];
mjj_sig_n2_cat2[5.0, 1.0, 100.0];
mjj_sig_gm0_cat2[122.0, 110, 130];
mjj_sig_gsigma_cat2[25.0, 10.0, 60.0];
mjj_sig_frac_cat2[0.1, 0, 1];

mjjGaussSig_cat2 = Gaussian(mjj, mjj_sig_gm0_cat2, mjj_sig_gsigma_cat2);
mjjCBSig_cat2 = CBShape(mjj, mjj_sig_m0_cat2, mjj_sig_sigma_cat2, mjj_sig_alpha1_cat2, mjj_sig_n1_cat2);
mjjSig_cat2 = AddPdf(mjjGaussSig_cat2, mjjCBSig_cat2, mjj_sig_frac_cat2);


CMS_hhbbgg_13TeV_mjj_bkg_par1_cat2[0.9, -100, 100];


mjj_hig_par1_ggh_cat2[0.1, 0, 10];
mjj_hig_par2_ggh_cat2[0.1, 0, 10];
mjj_hig_par3_ggh_cat2[0.1, 0, 10];

mjj_hig_par1_vbf_cat2[0.1, 0, 10];
mjj_hig_par2_vbf_cat2[0.1, 0, 10];
mjj_hig_par3_vbf_cat2[0.1, 0, 10];

mjj_hig_m0_tth_cat2[100, 70, 190];
mjj_hig_sigma_tth_cat2[50, 10, 100];
mjj_hig_alpha1_tth_cat2[1.0, 0.01, 10];
mjj_hig_n1_tth_cat2[1, 0.01, 10];
mjj_hig_alpha2_tth_cat2[1.0, 0.01, 10];
mjj_hig_n2_tth_cat2[1, 0.01, 10];
mjjHig_tth_cat2 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat2, mjj_hig_sigma_tth_cat2, mjj_hig_alpha1_tth_cat2, mjj_hig_n1_tth_cat2, mjj_hig_alpha2_tth_cat2, mjj_hig_n2_tth_cat2);

mjj_hig_m0_vh_cat2[100, 70, 190];
mjj_hig_sigma_vh_cat2[50, 10, 100];
mjj_hig_alpha1_vh_cat2[1.0, 0.01, 10];
mjj_hig_n1_vh_cat2[1, 0.01, 10];
mjj_hig_alpha2_vh_cat2[1.0, 0.01, 10];
mjj_hig_n2_vh_cat2[1, 0.01, 10];
mjjHig_vh_cat2 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat2, mjj_hig_sigma_vh_cat2, mjj_hig_alpha1_vh_cat2, mjj_hig_n1_vh_cat2, mjj_hig_alpha2_vh_cat2, mjj_hig_n2_vh_cat2);

mjj_hig_m0_bbh_cat2[100, 10, 180];
mjj_hig_sigma_bbh_cat2[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat2[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat2[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat2[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat2[1, 0.01, 10];
mjjHig_bbh_cat2 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat2, mjj_hig_sigma_bbh_cat2, mjj_hig_alpha1_bbh_cat2, mjj_hig_n1_bbh_cat2, mjj_hig_alpha2_bbh_cat2, mjj_hig_n2_bbh_cat2);